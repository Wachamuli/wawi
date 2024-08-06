mod binding;
mod panel;
mod styling;
mod widget;

use futures::StreamExt;
use panel::control_center::ControlCenter;
use widget::icon::icon;

use iced::{
    alignment,
    widget::{button, container, row, text, tooltip},
    Command, Element,
};

use iced_layershell::{reexport::Anchor, Application as _};

struct Bar {
    percentage: f64,
}

#[derive(Debug, Clone)]
enum Message {
    UPowerDevice(binding::upower::BatteryInfo),
    OpenControlCenter,
    CloseControlCenter,
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
                    binding::upower::BatteryInfo::Update {
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
            Message::OpenControlCenter => {
                ControlCenter::run(iced_layershell::settings::Settings {
                    antialiasing: true,
                    default_font: styling::font::SF_PRO,
                    layer_settings: iced_layershell::settings::LayerShellSettings {
                        layer: iced_layershell::reexport::Layer::Overlay,
                        anchor: Anchor::Right | Anchor::Top,
                        margins: (45, 10, 0, 0),
                        size: Some((500, 400)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .expect("Don't crash, please.");

                Command::none()
            }
            Message::CloseControlCenter => iced::window::close(iced::window::Id::MAIN),
            // Message::OpenCalendar => todo!(),
            // Message::CloseCalendar => todo!(),
            // Message::OpenNotificationsCenter => todo!(),
            // Message::CloseNotificationsCenter => todo!(),
        }
    }

    fn view(&self) -> Element<Message, Self::Theme> {
        let datetime = chrono::Utc::now().format("%a %d %I:%M %p");
        let date_display = iced::widget::text(datetime)
            .font(styling::font::SF_PRO_BOLD)
            .size(18);

        let left_section = container("Apps")
            .width(iced::Length::Fill)
            .align_y(alignment::Vertical::Center)
            .align_x(alignment::Horizontal::Left);

        let center_section = container("Workspaces")
            .width(iced::Length::Fill)
            .align_y(alignment::Vertical::Center)
            .align_x(alignment::Horizontal::Center);

        let right_section = container(
            row![
                button(tooltip(
                    iced::widget::svg(iced::widget::svg::Handle::from_path(format!(
                        "{}/assets/icons/battery-{}.svg",
                        env!("CARGO_MANIFEST_DIR"),
                        (self.percentage as i32 + 5) / 10 * 10
                    )))
                    .style(styling::style::Svg::Icon)
                    .width(20)
                    .height(20),
                    text(format!("Battery {}%", self.percentage)),
                    tooltip::Position::Bottom
                ))
                .style(styling::style::Button::Invisible)
                .on_press(Message::OpenControlCenter),
                button(tooltip(
                    icon(include_bytes!("../assets/icons/wififull.svg")),
                    "Connected to Crisel22",
                    tooltip::Position::Bottom
                ))
                .style(styling::style::Button::Invisible)
                .on_press(Message::CloseControlCenter),
                date_display,
                tooltip(
                    icon(include_bytes!("../assets/icons/bell.svg")),
                    "Notifications",
                    tooltip::Position::Bottom
                ),
            ]
            .spacing(20),
        )
        .width(iced::Length::Fill)
        .align_y(alignment::Vertical::Center)
        .align_x(iced::alignment::Horizontal::Right);

        container(row![left_section, center_section, right_section])
            .height(iced::Length::Fill)
            .width(iced::Length::Fill)
            .padding([0, 8, 0, 8])
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
