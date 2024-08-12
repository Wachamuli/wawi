use iced::futures::{self, FutureExt, StreamExt};

#[zbus::proxy(
    interface = "org.freedesktop.UPower.PowerProfiles",
    default_path = "/org/freedesktop/UPower/PowerProfiles",
    assume_defaults = true
)]
trait PowerProfiles {
    #[zbus(property)]
    fn active_profile(&self) -> zbus::Result<String>;

    // Not very useful, the data is given in a dynamic dict.
    #[zbus(property)]
    fn profiles(&self) -> zbus::Result<Vec<std::collections::HashMap<String, String>>>;

    #[zbus(property)]
    fn performance_degraded(&self) -> zbus::Result<String>;
}

// TODO: Actually, you should default to Unknown
#[derive(Debug, Clone, Default)]
pub enum PowerProfile {
    PowerSaver,
    #[default]
    Balanced,
    Performance,
    Unknown,
}

impl std::fmt::Display for PowerProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PowerProfile::PowerSaver => write!(f, "Power Saver"),
            PowerProfile::Balanced => write!(f, "Balanced"),
            PowerProfile::Performance => write!(f, "Performance"),
            PowerProfile::Unknown => write!(f, "Unknown"),
        }
    }
}

impl From<String> for PowerProfile {
    fn from(value: String) -> Self {
        match value.as_str() {
            "power-saver" => Self::PowerSaver,
            "balanced" => Self::Balanced,
            "performance" => Self::Performance,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub enum PowerProfileInfo {
    Active(PowerProfile),
}

async fn connection() -> zbus::Result<PowerProfilesProxy<'static>> {
    let connection = zbus::Connection::system().await?;
    let power_profiles = PowerProfilesProxy::new(&connection).await?;

    Ok(power_profiles)
}

pub async fn event_stream() -> zbus::Result<impl futures::Stream<Item = PowerProfileInfo>> {
    let power_profiles = connection().await?;
    let stream = power_profiles.receive_active_profile_changed().await;

    Ok(stream.map(move |_| {
        PowerProfileInfo::Active(PowerProfile::from(
            power_profiles
                .cached_active_profile()
                .unwrap_or_default()
                .unwrap_or_default(),
        ))
    }))
}

pub fn subscription<I>(id: I) -> iced::Subscription<PowerProfileInfo>
where
    I: 'static + std::hash::Hash,
{
    iced::subscription::run_with_id(
        id,
        async move {
            match event_stream().await {
                Ok(stream) => stream,
                Err(err) => {
                    eprintln!("An error has ocurred: {err}");
                    futures::future::pending().await
                }
            }
        }
        .flatten_stream(),
    )
}
