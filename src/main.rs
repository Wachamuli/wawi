mod binding;
mod panel;
mod styling;
mod widget;

use iced::{window, Application as _, Settings, Size};

fn main() -> iced::Result {
    panel::ControlCenter::run(Settings {
        id: Some("endless".to_string()),
        default_font: styling::font::SF_PRO,
        antialiasing: true,
        window: window::Settings {
            decorations: false,
            size: Size::new(475.0, 375.0),
            transparent: true,
            resizable: false,
            level: window::Level::AlwaysOnTop,
            position: iced::window::Position::Centered,
            exit_on_close_request: false,
            ..Default::default()
        },
        ..Default::default()
    })?;

    Ok(())
}

// use iced::futures::StreamExt;

// #[tokio::main]
// async fn main() -> zbus::Result<()> {
//     let conn = zbus::Connection::session().await?;
//     let device = binding::logind::DisplayBrightnessDeviceProxy::new(&conn).await?;
//     let mut stream = device.receive_current_brightness_changed().await;

//     while let Some(current) = stream.next().await {
//         println!("{:?}", current.get().await);
//     }

//     Ok(())
// }

// #[tokio::main]
// async fn main() -> zbus::Result<()> {
//     let backlights = binding::logind::scan_backlights()?;
//     let brightness_device = binding::logind::choose_brightness_device(backlights).await;
//     let display_device = binding::logind::DisplayBrightnessDevice::new(brightness_device);

//     let _conn = zbus::ConnectionBuilder::session()?
//         .name("org.zbus.MyGreeter")?
//         .serve_at("/org/zbus/MyGreeter", display_device)?
//         .build()
//         .await?;

//     std::future::pending::<()>().await;

//     Ok(())
// }
