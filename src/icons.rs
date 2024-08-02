use std::borrow::Cow;

use iced::{
    widget::{container, svg, text, tooltip, Svg},
    Element,
};

pub fn hint<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    hint: Option<&'a str>,
) -> Element<'a, Message> {
    if let Some(t) = hint {
        return tooltip(
            content,
            container(text(t))
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
        .into();
    }

    content.into()
}

fn icon(bytes: Cow<'static, [u8]>) -> Svg {
    svg(svg::Handle::from_memory(bytes)).width(20).height(20)
}

pub fn battery_indicator() -> Svg {
    svg(svg::Handle::from_memory(include_bytes!("../assets/icons/battery-90.svg")))
        .width(20)
        .height(20)
}

pub fn wifi_indicator() -> Svg {
    svg(svg::Handle::from_memory(include_bytes!("../assets/icons/wififull.svg")))
        .width(20)
        .height(20)
}

pub fn bell_icon() -> Svg {
    svg(svg::Handle::from_memory(include_bytes!("../assets/icons/bell.svg")))
        .width(20)
        .height(20)
}
