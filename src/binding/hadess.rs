use futures::{FutureExt, StreamExt};

#[zbus::proxy(
    interface = "org.freedesktop.UPower.PowerProfiles",
    default_path = "/org/freedesktop/UPower/PowerProfiles",
    assume_defaults = true
)]
trait PowerProfiles {
    #[zbus(property)]
    fn active_profile(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn profiles(&self) -> zbus::Result<Vec<zbus::zvariant::Dict>>;
}

#[derive(Debug, Clone)]
pub enum PowerModeInfo {
    Active(String),
}

async fn connection() -> zbus::Result<PowerProfilesProxy<'static>> {
    let connection = zbus::Connection::system().await?;
    let power_profiles = PowerProfilesProxy::new(&connection).await?;

    Ok(power_profiles)
}

async fn event_stream() -> zbus::Result<impl futures::Stream<Item = PowerModeInfo>> {
    let power_profiles = connection().await?;
    let stream = power_profiles.receive_active_profile_changed().await;

    Ok(stream.map(move |_| {
        PowerModeInfo::Active(
            power_profiles
                .cached_active_profile()
                .unwrap_or_default()
                .unwrap_or_default(),
        )
    }))
}

pub fn subscription<I>(id: I) -> iced::Subscription<PowerModeInfo>
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

// pub async fn get_profile_modes() -> zbus::Result<String> {
//     let power_profiles = connection().await?;
//     let profiles = power_profiles.profiles().await?;

//     profiles.await
// }
