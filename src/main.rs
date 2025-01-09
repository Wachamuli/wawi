mod binding;
mod panel;
mod styling;
mod widget;

// use iced::{window, Application as _, Settings, Size};

// fn main() -> iced::Result {
//     panel::ControlCenter::run(Settings {
//         id: Some("endless".to_string()),
//         default_font: styling::font::SF_PRO,
//         antialiasing: true,
//         window: window::Settings {
//             decorations: false,
//             size: Size::new(475.0, 675.0), // 375.0
//             transparent: true,
//             resizable: false,
//             level: window::Level::AlwaysOnTop,
//             position: iced::window::Position::Centered,
//             exit_on_close_request: false,
//             ..Default::default()
//         },
//         ..Default::default()
//     })?;
//
//     Ok(())
// }

use binding::logind::BrightnessDevice;
use iced::futures::StreamExt;

struct Bright;

#[zbus::interface(name = "org.zbus.MyGreeter")]
impl Bright {
    #[zbus(property)]
    fn current_brightness(&self) -> i32 {
        std::fs::read_to_string("/sys/class/backlight/intel_backlight/brightness")
            .unwrap()
            .trim()
            .parse()
            .unwrap()
    }

    // #[zbus(property)]
    // async fn current_brightness(&self) -> i32 {
    //     tokio::fs::read_to_string("/sys/class/backlight/intel_backlight/brightness")
    //         .await
    //         .unwrap()
    //         .trim()
    //         .parse()
    //         .unwrap()
    // }
}

#[zbus::proxy(
    interface = "org.zbus.MyGreeter",
    default_path = "/org/zbus/MyGreeter",
    assume_defaults = true
)]
trait BrightX {
    #[zbus(property)]
    fn current_brightness(&self) -> zbus::Result<i32>;
}

// Runs the DisplayBrightnessDevice server
#[tokio::main]
async fn main() -> zbus::Result<()> {
    // let backlights = binding::logind::scan_backlights()?;
    // let brightness_device = binding::logind::choose_brightness_device(backlights).await;
    // let display_device = binding::logind::DisplayBrightnessDevice::new(brightness_device);

    // let _conn = zbus::ConnectionBuilder::session()?
    //     .name("org.zbus.MyGreeter")?
    //     .serve_at("/org/zbus/MyGreeter", display_device)?
    //     .build()
    //     .await?;

    // println!("DBUS: Running DisplayBrightnessDevice server");

    // std::future::pending::<()>().await;

    // Ok(())

    let _conn = zbus::ConnectionBuilder::session()?
        .name("org.zbus.MyGreeter")?
        .serve_at("/org/zbus/MyGreeter", Bright)?
        .build()
        .await?;

    println!("DBUS: Running DisplayBrightnessDevice server");

    let conn = zbus::Connection::session().await?;
    let device = BrightXProxy::new(&conn).await?;
    let mut stream = device.receive_current_brightness_changed().await;

    while let Some(current) = stream.next().await {
        println!("{:?}", current.get().await);
    }

    std::future::pending::<()>().await;

    Ok(())
}
