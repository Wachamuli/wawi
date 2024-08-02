mod icons;

use iced::{
    widget::{container, row},
    Command, Element,
};
use iced_layershell::{reexport::Anchor, settings::LayerShellSettings, Application};

struct Bar;

// TODO: Remove this
#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Message {
    OpenControlCenter,
    CloseControlCenter,
    OpenCalendar,
    CloseCalendar,
    OpenNotificationsCenter,
    CloseNotificationsCenter
}

impl Application for Bar {
    type Message = Message;
    type Flags = ();
    type Theme = iced::Theme;
    type Executor = iced::executor::Default;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self, Command::none())
    }

    fn namespace(&self) -> String {
        String::from("morpheus")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::OpenControlCenter => todo!(),
            Message::CloseControlCenter => todo!(),
            Message::OpenCalendar => todo!(),
            Message::CloseCalendar => todo!(),
            Message::OpenNotificationsCenter => todo!(),
            Message::CloseNotificationsCenter => todo!(),
        }
    }

    fn view(&self) -> Element<Message> {
        let date: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
        let formatted_date = date.format("%a %d %I:%M %p");
        let date_display = iced::widget::text(formatted_date);

        let left_section = container("Apps")
            .width(iced::Length::Fill)
            .align_x(iced::alignment::Horizontal::Left);

        let center_section = container("Workspaces")
            .width(iced::Length::Fill)
            .align_x(iced::alignment::Horizontal::Center);

        let right_section = container(row![
            icons::battery_indicator(),
            icons::wifi_indicator(),
            date_display,
            icons::bell_icon()
        ].spacing(20))
        .width(iced::Length::Fill)
        .align_x(iced::alignment::Horizontal::Right);

        container(row![left_section, center_section, right_section])
            .padding([0, 8, 0, 8])
            .width(iced::Length::Fill)
            .style(container::Appearance {
                text_color: Some(iced::Color::from_rgb(1.0, 1.0, 1.0)),
                background: Some(iced::Background::Color(iced::Color::from_rgb(
                    0.0, 0.0, 0.0,
                ))),
                ..Default::default()
            })
            .into()
    }
}

fn main() -> Result<(), iced_layershell::Error> {
    Bar::run(iced_layershell::settings::Settings {
        layer_settings: LayerShellSettings {
            size: Some((0, 40)),
            exclusize_zone: 40,
            anchor: Anchor::Top | Anchor::Left | Anchor::Right,
            ..Default::default()
        },
        ..Default::default()
    })
}
