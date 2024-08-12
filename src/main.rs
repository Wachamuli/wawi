mod binding;
mod styling;
mod widget;

use iced::{
    futures::StreamExt,
    widget::{button, column, container, row, slider, svg, text},
    Alignment, Command, Element, Length,
};
use iced_layershell::{
    reexport::{Anchor, Layer},
    Application as _,
};

struct ControlCenter {
    is_battery_available: bool,
    on_battery: bool,
    percentage: f64,
    time_to_empty: i64,

    master_volume: u32,

    current_brightness: i32,
    max_brightness: i32,
    min_brightness: i32,

    active_power_profile: binding::hadess::PowerProfile,
}

#[derive(Debug, Clone)]
enum Message {
    UPowerDevice(binding::upower::BatteryInfo),
    HadessDevice(binding::hadess::PowerProfileInfo),
    ScreenDevice(binding::logind::DisplayInfo),

    SetMasterVolume(u32),
    SetBrightness(i32),
    GetBrightness(i32),
    ToggleProfiles,
}

impl iced_layershell::Application for ControlCenter {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = styling::theme::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                is_battery_available: true,
                on_battery: true,
                percentage: 100.0,
                time_to_empty: 0,

                master_volume: 100,

                current_brightness: 0,
                max_brightness: 0,
                min_brightness: 0,

                active_power_profile: binding::hadess::PowerProfile::Balanced,
            },
            Command::none(),
        )
    }

    fn namespace(&self) -> String {
        String::from("morpheus")
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::Subscription::batch([
            binding::upower::subscription(0).map(Message::UPowerDevice),
            binding::hadess::subscription(0).map(Message::HadessDevice),
            binding::logind::subscription(0).map(Message::ScreenDevice),
        ])
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::UPowerDevice(event) => match event {
                binding::upower::BatteryInfo::NotAvailable => {
                    self.is_battery_available = false;
                }
                binding::upower::BatteryInfo::Available {
                    on_battery,
                    percent,
                    time_to_empty,
                } => {
                    self.on_battery = on_battery;
                    self.percentage = percent;
                    self.time_to_empty = time_to_empty;
                }
            },
            Message::HadessDevice(event) => match event {
                binding::hadess::PowerProfileInfo::Active(profile) => {
                    self.active_power_profile = profile;
                }
            },
            // Message::SelectPowerProfile(_profile) => {
            //     todo!("Maybe use std::process::Command to set the power profile?")
            // }
            Message::ScreenDevice(event) => match event {
                binding::logind::DisplayInfo::Update {
                    current_brightness,
                    max_brightness,
                    min_brightness,
                } => {
                    self.current_brightness = current_brightness;
                    self.max_brightness = max_brightness;
                    self.min_brightness = min_brightness;
                }
            },
            Message::SetBrightness(value) => {
                let command = binding::logind::set_brightness(value);
                return Command::perform(command, Message::GetBrightness);
            }
            Message::GetBrightness(value) => {
                self.current_brightness = value as i32;
            }
            // Message::SetMasterVolume(volume) => {
            //     self.master_volume = volume;
            // }
            Message::ToggleProfiles => {
                println!("Toggle Profiles");
            }
            Message::SetMasterVolume(value) => {
                self.master_volume = value;
                println!("Setting master volume: {value}");
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message, Self::Theme> {
        let battery_icon = format!(
            "{}/assets/icons/battery{}-{}.svg",
            env!("CARGO_MANIFEST_DIR"),
            if !self.on_battery { "-charging" } else { "" },
            (self.percentage as i32 + 5) / 10 * 10
        );
        let wifi_icon = format!("{}/assets/icons/wifi-full.svg", env!("CARGO_MANIFEST_DIR"));
        let blue_icon = format!("{}/assets/icons/bluetooth.svg", env!("CARGO_MANIFEST_DIR"));
        let plane_icon = format!("{}/assets/icons/airplane.svg", env!("CARGO_MANIFEST_DIR"));
        let power_icon = format!("{}/assets/icons/power-mode.svg", env!("CARGO_MANIFEST_DIR"));
        let fan_icon = format!("{}/assets/icons/fan.svg", env!("CARGO_MANIFEST_DIR"));
        let volume_icon = format!("{}/assets/icons/volume-up.svg", env!("CARGO_MANIFEST_DIR"));
        let bright_icon = format!("{}/assets/icons/brightness.svg", env!("CARGO_MANIFEST_DIR"));

        // TODO: Network Manager, add power profile dropdown menu, change background to transparent, add degraded performance to power profile
        let icon = |icon_path| svg(svg::Handle::from_path(icon_path)).width(25).height(25);

        let circular_button = |icon_path| {
            button(icon(icon_path))
                .style(styling::style::Button::Circular)
                .padding(17)
        };

        let rectangular_button = |title, subtitle, icon_path, message| {
            button(
                row![
                    icon(icon_path),
                    column![
                        text(title).font(styling::font::SF_PRO_BOLD).size(16),
                        text(subtitle)
                    ]
                ]
                .spacing(20)
                .align_items(Alignment::Center),
            )
            .on_press(message)
            .width(250)
            .padding([10, 20, 10, 20])
        };

        let battery: Element<Message, Self::Theme> = if !self.is_battery_available {
            text("No Battery").into()
        } else {
            row![
                icon(battery_icon),
                column![
                    text(format!("{}%", self.percentage)).font(styling::font::SF_PRO_BOLD),
                    text(styling::format::seconds_to_hour_minute(self.time_to_empty))
                ],
            ]
            .spacing(10)
            .align_items(Alignment::Center)
            .into()
        };

        container(
            column![
                row![
                    container(battery).width(Length::Fill),
                    container(
                        row![
                            circular_button(wifi_icon),
                            circular_button(blue_icon),
                            circular_button(plane_icon),
                        ]
                        .spacing(10)
                    )
                ]
                .align_items(Alignment::Center),
                container(
                    column![
                        row![
                            icon(volume_icon),
                            slider(0..=100, self.master_volume, Message::SetMasterVolume)
                        ]
                        .align_items(Alignment::Center)
                        .spacing(10),
                        row![
                            icon(bright_icon),
                            slider(
                                self.min_brightness..=self.max_brightness,
                                self.current_brightness,
                                Message::SetBrightness
                            ),
                        ]
                        .align_items(Alignment::Center)
                        .spacing(10)
                    ]
                    .spacing(10)
                ),
                container(
                    column![
                        rectangular_button(
                            "Power Mode",
                            &self.active_power_profile.clone().into(),
                            power_icon,
                            Message::ToggleProfiles
                        ),
                        rectangular_button(
                            "Fan Profile",
                            &"Silent".to_string(),
                            fan_icon,
                            Message::ToggleProfiles
                        ),
                    ]
                    .spacing(10)
                )
            ]
            .spacing(20),
        )
        .style(styling::style::Container::HeavyRounded)
        .padding(32)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .into()
    }
}

fn main() -> Result<(), iced_layershell::Error> {
    ControlCenter::run(iced_layershell::settings::Settings {
        id: Some(String::from("control_center")),
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

// use iced::futures::StreamExt;

// #[tokio::main]
// async fn main() -> zbus::Result<()> {
//     if let Ok(mut stream) = binding::logind::event_stream().await {
//         while let Some(s) = stream.next().await {
//             println!("{:?}", s)
//         }
//     }

//     Ok(())
// }

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
