mod binding;
mod panel;
mod styling;
mod widget;

use iced_layershell::{
    reexport::{Anchor, Layer},
    Application as _,
};

fn main() -> Result<(), iced_layershell::Error> {
    panel::ControlCenter::run(iced_layershell::settings::Settings {
        id: Some("control_center".to_string()),
        antialiasing: true,
        default_font: styling::font::SF_PRO,
        layer_settings: iced_layershell::settings::LayerShellSettings {
            layer: Layer::Top,
            anchor: Anchor::Right | Anchor::Top,
            margins: (40 + 15, 10, 0, 0),
            size: Some((475, 375)),
            ..Default::default()
        },
        ..Default::default()
    })
}

// mod binding;

// #[tokio::main]
// async fn main() -> zbus::Result<()> {
//     let display_brightness_device = binding::logind::get_brightness_device().await;
//     let device = binding::logind::DisplayBrightnessDevice::new(display_brightness_device);

//     let connection = zbus::Connection::session().await?;
//     connection
//         .object_server()
//         .at("/org/morpheus/DisplayBrightnessDevice", device)
//         .await?;
//     connection
//         .request_name("org.morpheus.DisplayBrightnessDevice")
//         .await?;

//     loop {
//         // do something else, wait forever or timeout here:
//         // handling D-Bus messages is done in the background
//         std::future::pending::<()>().await;
//     }
// }
