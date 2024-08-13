use iced::futures::{self, FutureExt, StreamExt};
use std::io;

#[zbus::proxy(
    default_service = "org.freedesktop.login1",
    interface = "org.freedesktop.login1.Session",
    default_path = "/org/freedesktop/login1/session/auto"
)]
trait LogindSession {
    fn set_brightness(&self, subsystem: &str, name: &str, brightness: u32) -> zbus::Result<()>;
}

async fn connection() -> zbus::Result<LogindSessionProxy<'static>> {
    let conn = zbus::Connection::system().await?;
    let logind = LogindSessionProxy::new(&conn).await?;

    Ok(logind)
}

/// [`subsystem`] can be either `backlight` or `leds`.
#[derive(Debug, Clone)]
pub struct BrightnessDevice {
    subsystem: &'static str,
    sysname: String,
    max_brightness: u32,
    min_brightness: u32,
}

impl BrightnessDevice {
    pub async fn new(subsystem: &'static str, sysname: String) -> io::Result<Self> {
        let path = format!("/sys/class/{}/{}/max_brightness", subsystem, sysname);
        let value = tokio::fs::read_to_string(&path).await?;
        let max_brightness: u32 = value
            .trim()
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(Self {
            subsystem,
            sysname,
            max_brightness,
            min_brightness: (max_brightness as f32 * 0.1) as u32,
        })
    }

    pub async fn brightness(&self) -> io::Result<u32> {
        let path = format!("/sys/class/backlight/{}/brightness", &self.sysname);
        let value = tokio::fs::read_to_string(&path).await?;
        let brightness: u32 = value
            .trim()
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(brightness)
    }

    pub fn max_brightness(&self) -> u32 {
        self.max_brightness
    }

    pub fn min_brightness(&self) -> u32 {
        self.min_brightness
    }

    pub async fn set_brightness(&self, value: u32) -> u32 {
        let clamped_value = value.clamp(self.min_brightness, self.max_brightness);

        if let Ok(logind) = connection().await {
            let _ = logind
                .set_brightness(&self.subsystem, &self.sysname, clamped_value)
                .await;
        }

        clamped_value
    }
}

pub fn scan_backlights() -> io::Result<Vec<udev::Device>> {
    let mut udev_enumerator = udev::Enumerator::new()?;
    udev_enumerator.match_subsystem("backlight")?;
    let backlights: Vec<udev::Device> = udev_enumerator.scan_devices()?.collect();

    Ok(backlights)
}

async fn choose_brightness_device(backlights: Vec<udev::Device>) -> Option<BrightnessDevice> {
    let mut best_brightness_device = None;
    let mut best_brightness_value = 0;

    for backlight in backlights {
        let Some(sysname) = backlight.sysname().to_str() else {
            continue;
        };

        match BrightnessDevice::new("backlight", sysname.to_owned()).await {
            Ok(device) => {
                if device.max_brightness() > best_brightness_value {
                    best_brightness_value = device.max_brightness();
                    best_brightness_device = Some(device);
                }
            }
            Err(err) => eprint!("Couldn't setup the brightness device: {err}"),
        }
    }

    best_brightness_device
}

pub async fn get_brightness_device() -> Option<BrightnessDevice> {
    let backligths = scan_backlights().ok()?;
    let brightnes_device = choose_brightness_device(backligths).await;
    brightnes_device
}

pub struct DisplayBrightnessDevice {
    display_brightness_device: Option<BrightnessDevice>,
}

impl DisplayBrightnessDevice {
    pub fn new(display_brightness_device: Option<BrightnessDevice>) -> Self {
        Self {
            display_brightness_device,
        }
    }
}

#[zbus::interface(
    name = "org.morpheus.DisplayBrightnessDevice",
    proxy(
        gen_blocking = false,
        default_service = "org.morpheus.DisplayBrightnessDevice",
        default_path = "/org/morpheus/DisplayBrightnessDevice",
    )
)]
impl DisplayBrightnessDevice {
    #[zbus(property)]
    async fn current_brightness(&self) -> i32 {
        let Some(brightness_device) = &self.display_brightness_device else {
            return -1;
        };
        let Ok(current_brightness) = brightness_device.brightness().await else {
            return -1;
        };

        current_brightness as i32
    }

    #[zbus(property)]
    fn max_brightness(&self) -> i32 {
        match &self.display_brightness_device {
            Some(t) => t.max_brightness() as i32,
            None => -1,
        }
    }

    #[zbus(property)]
    fn min_brightness(&self) -> i32 {
        match &self.display_brightness_device {
            Some(t) => t.min_brightness() as i32,
            None => -1,
        }
    }

    async fn set_brightness(&self, value: u32) {
        if let Some(device) = &self.display_brightness_device {
            device.set_brightness(value).await;
        }
    }
}

#[derive(Debug, Clone)]
pub enum DisplayInfo {
    Available {
        current_brightness: i32,
        max_brightness: i32,
        min_brightness: i32,
    },
}

async fn conn() -> zbus::Result<DisplayBrightnessDeviceProxy<'static>> {
    let conn = zbus::Connection::session().await?;
    let device = DisplayBrightnessDeviceProxy::new(&conn).await?;

    Ok(device)
}

pub async fn set_brightness(value: i32) -> i32 {
    if let Ok(device) = conn().await {
        device
            .set_brightness(value as u32)
            .await
            .unwrap_or_else(|err| eprintln!("Couldn't set the display brightness: {err}"));
    }

    value
}

async fn event_stream() -> zbus::Result<impl futures::Stream<Item = DisplayInfo>> {
    let device = conn().await?;
    let stream = device.receive_current_brightness_changed().await;

    Ok(stream.map(move |_| DisplayInfo::Available {
        current_brightness: device
            .cached_current_brightness()
            .unwrap_or_default()
            .unwrap_or_default(),
        max_brightness: device
            .cached_max_brightness()
            .unwrap_or_default()
            .unwrap_or_default(),
        min_brightness: device
            .cached_min_brightness()
            .unwrap_or_default()
            .unwrap_or_default(),
    }))
}

pub fn subscription<T>(id: T) -> iced::Subscription<DisplayInfo>
where
    T: 'static + std::hash::Hash,
{
    iced::subscription::run_with_id(
        id,
        async move {
            match event_stream().await {
                Ok(sst) => sst,
                Err(err) => {
                    eprintln!("An error has ocurred: {err}");
                    futures::future::pending().await
                }
            }
        }
        .flatten_stream(),
    )
}
