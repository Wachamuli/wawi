use crate::styling;
use iced::widget::{svg, Svg};

pub fn icon(bytes: &'static [u8]) -> Svg<styling::theme::Theme> {
    const ICON_SIZE: u16 = 20;

    svg(svg::Handle::from_memory(bytes))
        .style(styling::style::Svg::Icon)
        .width(ICON_SIZE)
        .height(ICON_SIZE)
}

// pub fn battery_indicator<'a, Message: 'a>() -> Tooltip<'a, Message, styling::theme::Theme> {
//     hint(
//         icon(include_bytes!("../../assets/icons/battery-90.svg")).into(),
//         "Battery 100%",
//     ).into()
// }

// pub fn wifi_indicator<'a, Message: 'a>() -> Element<'a, Message> {
//     hint(
//         icon(include_bytes!("../../assets/icons/wififull.svg")),
//         "Connected to Crisel22",
//     )
//     .into()
// }

// pub fn bell_icon<'a, Message: 'a>() -> Element<'a, Message> {
//     hint(
//         icon(include_bytes!("../../assets/icons/bell.svg")),
//         "Notifications",
//     )
//     .into()
// }
