use crate::styling;
use iced::widget::{svg, Svg};

pub fn icon(bytes: &'static [u8]) -> Svg<styling::theme::Theme> {

    const ICON_SIZE: u16 = 20;

    svg(svg::Handle::from_memory(bytes))
        .style(styling::style::Svg::Icon)
        .width(ICON_SIZE)
        .height(ICON_SIZE)
}

struct Icon {
    path: std::path::PathBuf,
}

impl Icon {
    fn new(path: std::path::PathBuf) -> Self {
        Self { path }
    }
}
