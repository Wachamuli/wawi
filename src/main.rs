mod binding;
mod styling;
mod widget;

use iced::{
    widget::{button, column, container, row, svg, text},
    Alignment, Command, Element, Length,
};
use iced_layershell::{reexport::Anchor, Application as _};

struct ControlCenter {
    is_charging: bool,
    percentage: f64,
    time_to_empty: i64,

    power_mode: String,
}

#[derive(Debug, Clone)]
enum Message {
    UPowerDevice(binding::upower::BatteryInfo),
    Hadess(binding::hadess::PowerModeInfo),
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
            Message::UPowerDevice(event) => {
                match event {
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
                }

                Command::none()
            }
            Message::Hadess(event) => match event {
                binding::hadess::PowerModeInfo::Active(mode) => {
                    self.power_mode = mode;
                    Command::none()
                }
            },
        }
    }

    fn view(&self) -> Element<Message, Self::Theme> {
        let battery_icon = format!(
            "{}/assets/icons/battery-{}.svg",
            env!("CARGO_MANIFEST_DIR"),
            (self.percentage as i32 + 5) / 10 * 10
        );
        let wifi_icon = format!("{}/assets/icons/wifi-full.svg", env!("CARGO_MANIFEST_DIR"));
        let blue_icon = format!("{}/assets/icons/bluetooth.svg", env!("CARGO_MANIFEST_DIR"));
        let plane_icon = format!("{}/assets/icons/airplane.svg", env!("CARGO_MANIFEST_DIR"));
        let power_icon = format!("{}/assets/icons/power-mode.svg", env!("CARGO_MANIFEST_DIR"));

        // TODO: Power Profiles and Network Manager
        let rec_button = button(
            row![
                svg(svg::Handle::from_path(power_icon)).width(25).height(25),
                column![
                    text("Power Mode").font(styling::font::SF_PRO_BOLD).size(16),
                    text(kebab_to_pascal_case(&self.power_mode))
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
                            text(display_battery_time(self.time_to_empty))
                        ],
                    ]
                    .spacing(10)
                    .align_items(Alignment::Center),
                )
                .width(Length::Fill),
                container(
                    row![
                        button(svg(svg::Handle::from_path(wifi_icon)).width(25).height(25))
                            .padding(17)
                            .style(styling::style::Button::Circular),
                        button(svg(svg::Handle::from_path(blue_icon)).width(25).height(25))
                            .padding(17)
                            .style(styling::style::Button::Circular),
                        button(svg(svg::Handle::from_path(plane_icon)).width(25).height(25))
                            .padding(17)
                            .style(styling::style::Button::Circular),
                    ]
                    .spacing(10)
                )
            ]
            .align_items(Alignment::Center),
            container(column![rec_button])
        ])
        .padding(32)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .style(styling::style::Container::HeavyRounded)
        .into()
    }
}

fn display_battery_time(seconds: i64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;

    match (hours, minutes) {
        (h, _) if h > 1 => format!("{h} hours"),
        (_, m) if m > 1 => format!("{m} minutes"),
        (1, _) => format!("1 hour"),
        (_, 1) => format!("1 minute"),
        _ => String::from("Less than a minute"),
    }
}

fn kebab_to_pascal_case(string: &String) -> String {
    string
        .split("-")
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(n) => n.to_uppercase().to_string() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
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
