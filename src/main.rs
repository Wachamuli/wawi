mod binding;
mod styling;
mod widget;

use iced::{
    widget::{column, container, row, svg, text},
    Alignment, Command, Element,
};
use iced_layershell::{reexport::Anchor, Application as _};
use styling::font::SF_PRO_BOLD;

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

        container(
            row![
                svg(svg::Handle::from_path(battery_icon))
                    .width(25)
                    .height(25),
                column![
                    text(format!("{}%", self.percentage)).font(SF_PRO_BOLD),
                    text(format!("{} hours", self.time_to_empty / 3600))
                ]
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
