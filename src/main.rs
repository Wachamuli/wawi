mod binding;
mod panel;
mod styling;
mod widget;

use binding::upower::DeviceProxy;
use widget::icon::icon;

use iced::{
    alignment,
    widget::{button, container, row, text, tooltip},
    Command, Element,
};

use iced_layershell::{reexport::Anchor, Application};

struct Bar {
    percentange: f64,
}

#[derive(Debug, Clone)]
enum Message {
    BatteryState(zbus::Result<f64>),
    OpenControlCenter,
}

impl iced_layershell::Application for Bar {
    type Message = Message;
    type Flags = ();
    type Theme = styling::theme::Theme;
    type Executor = iced::executor::Default;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self { percentange: 100.0 },
            Command::perform(battery_state(), Message::BatteryState),
        )
    }

    fn namespace(&self) -> String {
        String::from("morpheus")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::BatteryState(state) => {
                if let Ok(percentage) = state {
                    self.percentange = percentage;
                }

                Command::none()
            }
            Message::OpenControlCenter => Command::none(),
            // Message::CloseControlCenter => Command::none(),
            // Message::OpenCalendar => todo!(),
            // Message::CloseCalendar => todo!(),
            // Message::OpenNotificationsCenter => todo!(),
            // Message::CloseNotificationsCenter => todo!(),
        }
    }

    fn view(&self) -> Element<Message, Self::Theme> {
        let datetime = chrono::Utc::now().format("%a %d %I:%M %p");
        let date_display = iced::widget::text(datetime);

        let left_section = container("Apps")
            .width(iced::Length::Fill)
            .align_x(alignment::Horizontal::Left);

        let center_section = container("Workspaces")
            .width(iced::Length::Fill)
            .align_x(alignment::Horizontal::Center);

        let right_section = container(
            row![
                button(tooltip(
                    icon(include_bytes!("../assets/icons/battery-90.svg")),
                    text(format!("Battery {}%", self.percentange)),
                    tooltip::Position::Bottom
                ))
                .on_press(Message::OpenControlCenter),
                tooltip(
                    icon(include_bytes!("../assets/icons/wififull.svg")),
                    "Connected to Crisel22",
                    tooltip::Position::Bottom
                ),
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
        .align_x(iced::alignment::Horizontal::Right);

        container(row![left_section, center_section, right_section])
            .height(iced::Length::Fill)
            .width(iced::Length::Fill)
            .padding([0, 8, 0, 8])
            .into()
    }
}

async fn battery_state<'a>() -> zbus::Result<f64> {
    let connection = zbus::Connection::system().await?;
    let upower = binding::upower::UPowerProxy::new(&connection).await?;
    let interface = upower.get_display_device().await?;

    interface.percentage().await
}

fn main() -> Result<(), iced_layershell::Error> {
    Bar::run(iced_layershell::settings::Settings {
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
