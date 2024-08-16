mod binding;
mod panel;
mod styling;
mod widget;

use iced_layershell::{
    reexport::{Anchor, Layer},
    Application as _,
};

// fn main() -> Result<(), iced_layershell::Error> {
//     panel::ControlCenter::run(iced_layershell::settings::Settings {
//         id: Some("control_center".to_string()),
//         antialiasing: true,
//         default_font: styling::font::SF_PRO,
//         layer_settings: iced_layershell::settings::LayerShellSettings {
//             layer: Layer::Top,
//             anchor: Anchor::Right | Anchor::Top,
//             margins: (40 + 15, 10, 0, 0),
//             size: Some((475, 375)),
//             ..Default::default()
//         },
//         ..Default::default()
//     })
// }

use iced::futures::StreamExt;

#[tokio::main]
async fn main() -> zbus::Result<()> {
    let conn = zbus::Connection::session().await?;
    let device = binding::logind::DisplayBrightnessDeviceProxy::new(&conn).await?;
    let mut stream = device.receive_current_brightness_changed().await;

    while let Some(current) = stream.next().await {
        println!("{:?}", current.get().await);
    }

    Ok(())
}

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
