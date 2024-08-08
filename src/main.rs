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
    is_charging: bool,
    percentage: f64,
    time_to_empty: i64,

    power_mode: String,
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
                percentage: 100.0,
                time_to_empty: 0,
                is_charging: false,

                power_mode: "Balanced".to_string(),
                power_profiles: Vec::new(),
            },
            Command::perform(binding::hadess::get_profile_modes(), Message::Hadess),
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
                    todo!("What to do when there's no battery in the system?")
                }
                binding::upower::BatteryInfo::Available {
                    is_charging,
                    percent,
                    time_to_empty,
                } => {
                    self.is_charging = is_charging;
                    self.percentage = percent;
                    self.time_to_empty = time_to_empty;
                }
            },
            Message::Hadess(event) => match event {
                binding::hadess::PowerProfileInfo::Active(mode) => {
                    self.power_mode = mode;
                }
                binding::hadess::PowerProfileInfo::Profiles(power_profiles) => {
                    self.power_profiles = power_profiles
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
            if self.is_charging { "-charging" } else { "" },
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

        let rectangular_button = |title, subtitle, icon_path, message| {
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

        container(
            column![
                row![
                    container(
                        row![
                            svg(svg::Handle::from_path(battery_icon))
                                .width(25)
                                .height(25),
                            column![
                                text(format!("{}%", self.percentage))
                                    .font(styling::font::SF_PRO_BOLD),
                                text(styling::format::seconds_to_hour_minute(self.time_to_empty))
                            ],
                        ]
                        .spacing(10)
                        .align_items(Alignment::Center),
                    )
                    .width(Length::Fill),
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
                            styling::format::kebab_to_title_case(&self.power_mode),
                            &power_icon,
                            Message::SelectPowerProfile
                        ),
                        rectangular_button(
                            "Fan Profile",
                            "Silent".to_string(),
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
