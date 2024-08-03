use crate::appearance;

use super::hint::hint;

use iced::{
    widget::{svg, Svg, Tooltip},
    Element,
};

pub fn icon(bytes: &'static [u8]) -> Svg<appearance::theme::Theme> {
    const ICON_SIZE: u16 = 20;

    svg(svg::Handle::from_memory(bytes))
        .width(ICON_SIZE)
        .height(ICON_SIZE)
}

// pub fn battery_indicator<'a, Message: 'a>() ->  impl Into<Element<'a, Message>> {
//     hint(
//         icon(include_bytes!("../../assets/icons/battery-90.svg")),
//         "Battery 100%",
//     )
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
