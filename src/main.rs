mod binding;
mod panel;
mod styling;
mod widget;

use iced::{
    widget::{container, text},
    Command, Element,
};

use iced_layershell::{reexport::Anchor, Application as _};

struct Bar {
    percentage: f64,
}

#[derive(Debug, Clone)]
enum Message {
    UPowerDevice(binding::upower::BatteryInfo),
}

impl iced_layershell::Application for Bar {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = styling::theme::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self { percentage: 100.0 }, Command::none())
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
                    binding::upower::BatteryInfo::Available {
                        is_charging,
                        percent,
                        time_to_empty,
                    } => {
                        self.percentage = percent;
                    }
                    binding::upower::BatteryInfo::NotAvailable => std::process::exit(0),
                }

                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message, Self::Theme> {
        container(text("Control Center"))
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .style(styling::style::Container::HeavyRounded)
            .into()
    }
}

fn main() -> Result<(), iced_layershell::Error> {
    Bar::run(iced_layershell::settings::Settings {
        antialiasing: true,
        default_font: styling::font::SF_PRO,
        layer_settings: iced_layershell::settings::LayerShellSettings {
            size: Some((0, 60)),
            exclusize_zone: 40,
            anchor: Anchor::Top | Anchor::Left | Anchor::Right,
            ..Default::default()
        },
        ..Default::default()
    })
}
