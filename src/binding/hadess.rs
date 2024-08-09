use iced::futures::{self, FutureExt, StreamExt};

#[zbus::proxy(
    interface = "org.freedesktop.UPower.PowerProfiles",
    default_path = "/org/freedesktop/UPower/PowerProfiles",
    assume_defaults = true
)]
trait PowerProfiles {
    #[zbus(property)]
    fn active_profile(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn profiles(&self) -> zbus::Result<Vec<std::collections::HashMap<String, String>>>;
}

#[derive(Debug, Clone)]
pub enum PowerProfile {
    PowerSaver,
    Balanced,
    Performance,
    Unknown,
}

impl std::fmt::Display for PowerProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PowerProfile::PowerSaver => write!(f, "{}", "Power Saver"),
            PowerProfile::Balanced => write!(f, "{}", "Balanced"),
            PowerProfile::Performance => write!(f, "{}", "Performance"),
            PowerProfile::Unknown => write!(f, "{}", "Unknown"),
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

impl Into<String> for PowerProfile {
    fn into(self) -> String {
        match self {
            PowerProfile::PowerSaver => "Power Saver".to_string(),
            PowerProfile::Balanced => "Balanced".to_string(),
            PowerProfile::Performance => "Performance".to_string(),
            PowerProfile::Unknown => "Unknown".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PowerProfileInfo {
    // Profiles(Vec<String>),
    Active(PowerProfile),
}

async fn connection() -> zbus::Result<PowerProfilesProxy<'static>> {
    let connection = zbus::Connection::system().await?;
    let power_profiles = PowerProfilesProxy::new(&connection).await?;

    Ok(power_profiles)
}

// pub async fn get_profile_modes() -> PowerProfileInfo {
//     // TODO: find a more ergonomic way to handle this.
//     let Ok(conn) = connection().await else {
//         eprintln!("DBUS: An error has ocurred stablishing the system connection.");
//         return PowerProfileInfo::Profiles(Vec::new());
//     };
//     let Ok(profiles) = conn.profiles().await else {
//         eprintln!("DBUS: An error has ocurred receiving the power profiles.");
//         return PowerProfileInfo::Profiles(Vec::new());
//     };

//     // FIXME: it returns empty strings instead of nothing when the given key is missing.
//     let power_profiles: Vec<String> = profiles
//         .iter()
//         .map(|f| match f.get("Profile") {
//             Some(profile) => profile.clone(),
//             None => String::new(),
//         })
//         .collect();

//     PowerProfileInfo::Profiles(power_profiles)
// }

async fn event_stream() -> zbus::Result<impl futures::Stream<Item = PowerProfileInfo>> {
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
