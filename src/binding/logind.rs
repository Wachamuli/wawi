#[zbus::proxy(
    default_service = "org.freedesktop.login1",
    interface = "org.freedesktop.login1.Session",
    default_path = "/org/freedesktop/login1/session/auto"
)]
trait LogindSession {
    fn set_brightness(&self, subsystem: &str, name: &str, brightness: u32) -> zbus::Result<()>;
}

pub async fn set_brightness(value: u32) -> u32 {
    let conn = zbus::Connection::system().await.unwrap();
    let logind = LogindSessionProxy::new(&conn).await.unwrap();

    let _ = logind
        .set_brightness("backlight", "intel_backlight", value)
        .await
        .unwrap();

    value
}
