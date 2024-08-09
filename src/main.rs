mod binding;
mod styling;
mod widget;

use iced::{
    widget::{button, column, container, row, svg, text},
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

    active_power_mode: binding::hadess::PowerProfile,
    power_profiles: Vec<String>,
}

#[derive(Debug, Clone)]
enum Message {
    UPowerDevice(binding::upower::BatteryInfo),
    Hadess(binding::hadess::PowerProfileInfo),
    SelectPowerProfile,
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

                active_power_mode: binding::hadess::PowerProfile::Balanced,
                power_profiles: Vec::new(),
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
            binding::hadess::subscription(0).map(Message::Hadess),
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
            Message::Hadess(event) => match event {
                binding::hadess::PowerProfileInfo::Active(mode) => {
                    self.active_power_mode = mode;
                }
            },
            Message::SelectPowerProfile => {
                println!("Power mode selected");
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

        // TODO: Network Manager, add power profile dropdown menu, change background to transparent

        let circular_button = |icon_path| {
            button(svg(svg::Handle::from_path(icon_path)).width(25).height(25))
                .style(styling::style::Button::Circular)
                .padding(17)
        };

        let rectangular_button = |title, subtitle: &String, icon_path, message| {
            button(
                row![
                    svg(svg::Handle::from_path(icon_path)).width(25).height(25),
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
                svg(svg::Handle::from_path(battery_icon))
                    .width(25)
                    .height(25),
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
                            circular_button(&wifi_icon),
                            circular_button(&blue_icon),
                            circular_button(&plane_icon),
                        ]
                        .spacing(10)
                    )
                ]
                .align_items(Alignment::Center),
                container(
                    column![
                        rectangular_button(
                            "Power Mode",
                            &self.active_power_mode.clone().into(),
                            &power_icon,
                            Message::SelectPowerProfile
                        ),
                        rectangular_button(
                            "Fan Profile",
                            &"Silent".to_string(),
                            &fan_icon,
                            Message::SelectPowerProfile
                        ),
                    ]
                    .spacing(10)
                )
            ]
            .spacing(10),
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
