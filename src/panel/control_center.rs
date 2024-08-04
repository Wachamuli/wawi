use iced::{
    widget::{button, column, container},
    Command,
};

use crate::{styling, widget::icon::icon};

pub struct ControlCenter;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Close,
}

impl iced_layershell::Application for ControlCenter {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = styling::theme::Theme;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self, Command::none())
    }

    fn namespace(&self) -> String {
        String::from("morpheus")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Close => iced::window::close(iced::window::Id::unique()),
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        container(column![
            button(
                icon(include_bytes!("../../assets/icons/wififull.svg"))
                .content_fit(iced::ContentFit::Cover)
                .width(25)
                .height(25)
            )
            .style(styling::style::Button::Circular)
            .width(75)
            .height(75)
            .on_press(Message::Close)
        ])
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .style(styling::style::Container::HeavyRounded)
        .into()
    }
}
