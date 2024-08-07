mod binding;
mod styling;
mod widget;

use std::io::Empty;

use iced::{
    widget::{button, column, container, row, svg, text},
    Alignment, Command, Element, Length,
};
use iced_layershell::{reexport::Anchor, Application as _};

struct ControlCenter {
    is_charging: bool,
    percentage: f64,
    time_to_empty: i64,
}

#[derive(Debug, Clone)]
enum Message {
    UPowerDevice(binding::upower::BatteryInfo),
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
            },
            Command::none(),
        )
    }

    fn namespace(&self) -> String {
        String::from("morpheus")
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        binding::upower::suscription(0).map(Message::UPowerDevice)
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

        // TODO: Power Profiles and Network Manager

        container(
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
        )
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
