mod binding;
mod styling;
mod widget;

use iced::{
    widget::{button, column, combo_box, container, row, svg, text},
    Alignment, Command, Element, Length,
};
use iced_layershell::{reexport::Anchor, Application as _};

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
    PowerModeSelected,
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
            Message::PowerModeSelected => {
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

        // TODO: Network Manager

        let rec_button = button(
            row![
                svg(svg::Handle::from_path(power_icon)).width(25).height(25),
                column![
                    text("Power Mode").font(styling::font::SF_PRO_BOLD).size(16),
                    text(styling::format::kebab_to_title_case(&self.power_mode))
                ]
            ]
            .spacing(20)
            .align_items(Alignment::Center),
        )
        .width(250)
        .padding([10, 20, 10, 20]);

        container(column![
            row![
                container(
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
                    .align_items(Alignment::Center),
                )
                .width(Length::Fill),
                container(
                    row![
                        button(svg(svg::Handle::from_path(wifi_icon)).width(25).height(25))
                            .style(styling::style::Button::Circular)
                            .padding(17),
                        button(svg(svg::Handle::from_path(blue_icon)).width(25).height(25))
                            .style(styling::style::Button::Circular)
                            .padding(17),
                        button(svg(svg::Handle::from_path(plane_icon)).width(25).height(25))
                            .style(styling::style::Button::Circular)
                            .padding(17),
                    ]
                    .spacing(10)
                )
            ]
            .align_items(Alignment::Center),
            container(column![rec_button])
        ])
        .style(styling::style::Container::HeavyRounded)
        .padding(32)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .into()
    }
}

fn main() -> Result<(), iced_layershell::Error> {
    ControlCenter::run(iced_layershell::settings::Settings {
        antialiasing: true,
        default_font: styling::font::SF_PRO,
        layer_settings: iced_layershell::settings::LayerShellSettings {
            size: Some((475, 375)),
            anchor: Anchor::Right | Anchor::Top,
            margins: (40 + 15, 10, 0, 0),
            ..Default::default()
        },
        ..Default::default()
    })
}
