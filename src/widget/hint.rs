use iced::{
    widget::{container, text, tooltip, Tooltip},
    Element,
};

pub fn hint<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    hint: &str,
) -> Tooltip<'a, Message> {
    tooltip(
        content,
        container(text(hint))
            .padding([10, 20, 10, 20])
            .style(container::Appearance {
                background: Some(iced::Background::Color(iced::Color::from_rgb(
                    0.0, 0.0, 0.0,
                ))),
                border: iced::Border {
                    width: 1.0,
                    radius: iced::border::Radius::from(10),
                    color: iced::Color::from_rgb(1.0, 1.0, 1.0),
                },
                text_color: Some(iced::Color::from_rgb(1.0, 1.0, 1.0)),
                ..Default::default()
            }),
        tooltip::Position::Bottom,
    )
}
