use iced::futures::{self, FutureExt, StreamExt};
use zbus::zvariant::OwnedValue;

#[derive(Debug, OwnedValue)]
pub enum BatteryState {
    Unknown = 0,
    Charging = 1,
    Discharging = 2,
    Empty = 3,
    FullyCharged = 4,
    PendingCharge = 5,
    PendingDischarge = 6,
}

#[derive(Debug, PartialEq, OwnedValue)]
pub enum BatteryType {
    Unknown = 0,
    LinePower = 1,
    Battery = 2,
    Ups = 3,
    Monitor = 4,
    Mouse = 5,
    Keyboard = 6,
    Pda = 7,
    Phone = 8,
}

#[derive(Debug, OwnedValue)]
pub enum BatteryLevel {
    Unknown = 0,
    None = 1,
    Low = 3,
    Critical = 4,
    Normal = 6,
    High = 7,
    Full = 8,
}

#[zbus::proxy(
    interface = "org.freedesktop.UPower.Device",
    default_service = "org.freedesktop.UPower",
    assume_defaults = false
)]
trait Device {
    #[zbus(property)]
    fn battery_level(&self) -> zbus::Result<BatteryLevel>;

    #[zbus(property)]
    fn capacity(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn energy(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn energy_empty(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn energy_full(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn energy_full_design(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn has_history(&self) -> zbus::Result<bool>;

    #[zbus(property)]
    fn has_statistics(&self) -> zbus::Result<bool>;

    #[zbus(property)]
    fn icon_name(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn is_present(&self) -> zbus::Result<bool>;

    #[zbus(property)]
    fn is_rechargeable(&self) -> zbus::Result<bool>;

    #[zbus(property)]
    fn luminosity(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn model(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn native_path(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn online(&self) -> zbus::Result<bool>;

    #[zbus(property)]
    fn percentage(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn power_supply(&self) -> zbus::Result<bool>;

    fn refresh(&self) -> zbus::Result<()>;

    #[zbus(property)]
    fn serial(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn state(&self) -> zbus::Result<BatteryState>;

    #[zbus(property)]
    fn temperature(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn time_to_empty(&self) -> zbus::Result<i64>;

    #[zbus(property)]
    fn time_to_full(&self) -> zbus::Result<i64>;

    #[zbus(property, name = "Type")]
    fn type_(&self) -> zbus::Result<BatteryType>;

    #[zbus(property)]
    fn vendor(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn voltage(&self) -> zbus::Result<f64>;
}

#[zbus::proxy(interface = "org.freedesktop.UPower", assume_defaults = true)]
trait UPower {
    #[zbus(object = "Device")]
    fn get_display_device(&self);

    #[zbus(property)]
    fn on_battery(&self) -> zbus::Result<bool>;

    fn enumerate_devices(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;
}

#[derive(Debug, Clone)]
pub enum BatteryInfo {
    NotAvailable,
    Available {
        on_battery: bool,
        percent: f64,
        time_to_empty: i64,
    },
}

async fn connection() -> zbus::Result<UPowerProxy<'static>> {
    let connection = zbus::Connection::system().await?;
    let upower = UPowerProxy::new(&connection).await?;

    Ok(upower)
}

async fn available_devices(
    upower: &UPowerProxy<'_>,
    devices: &Vec<zbus::zvariant::OwnedObjectPath>,
) -> Option<BatteryInfo> {
    let mut availability = Some(BatteryInfo::NotAvailable);

    for device in devices {
        let Ok(d) = DeviceProxy::builder(upower.inner().connection()).path(device) else {
            continue;
        };
        let Ok(d) = d.build().await else {
            continue;
        };

        if d.type_().await == Ok(BatteryType::Battery) && d.power_supply().await.unwrap_or_default()
        {
            availability = None;
            break;
        }
    }

    availability
}

async fn event_stream() -> zbus::Result<impl futures::Stream<Item = BatteryInfo>> {
    let upower = connection().await?;
    let devices = upower.enumerate_devices().await?;
    let device = upower.get_display_device().await?;

    let availability = available_devices(&upower, &devices).await;
    let initial = futures::stream::iter(availability);

    let stream = futures::stream_select!(
        device.receive_state_changed().await.map(|_| ()),
        device.receive_percentage_changed().await.map(|_| ()),
        device.receive_time_to_empty_changed().await.map(|_| ()),
    );

    Ok(initial.chain(stream.map(move |_| {
        BatteryInfo::Available {
            on_battery: upower
                .cached_on_battery()
                .unwrap_or_default()
                .unwrap_or_default(),
            percent: device
                .cached_percentage()
                .unwrap_or_default()
                .unwrap_or_default(),
            time_to_empty: device
                .cached_time_to_empty()
                .unwrap_or_default()
                .unwrap_or_default(),
        }
    })))
}

pub fn subscription<I>(id: I) -> iced::Subscription<BatteryInfo>
where
    I: 'static + std::hash::Hash,
{
    iced::subscription::run_with_id(
        id,
        async move {
            match event_stream().await {
                Ok(stream) => stream,
                Err(err) => {
                    // TODO: You should have a logger for this.
                    eprint!("An error has ocurred: {err}");
                    futures::future::pending().await
                }
            }
        }
        .flatten_stream(),
    )
}
