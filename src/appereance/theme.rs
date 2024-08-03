use iced::{border, Border, Color};


#[derive(Default, Debug, PartialEq, Eq, Copy, Clone)]
pub enum Theme {
    #[default]
    Dark,
    Light,
}

pub struct Palette {
    pub background: Color,
    pub foreground: Color,
    pub primary: Color,
    pub secondary: Color,
    pub surface: Color,
    pub error: Color,
    pub border: Border,
}

impl Theme {
    pub fn palette(self) -> Palette {
        match self {
            Theme::Light => todo!(),
            Theme::Dark => Palette {
                background: Color::BLACK,
                foreground: Color::WHITE,
                primary: Color::BLACK,
                secondary: Color::WHITE,
                surface: Color::WHITE,
                error: Color::WHITE,

                border: Border {
                    color: Color::WHITE,
                    radius: border::Radius::from(10),
                    width: 2.0,
                },
            },
        }
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "Morpheus")
    }
}

pub mod fonts {
    pub const SF_PRO: iced::Font = iced::Font {
        family: iced::font::Family::Name("SF Pro Rounded"),
        weight: iced::font::Weight::Normal,
        stretch: iced::font::Stretch::Normal,
        style: iced::font::Style::Normal,
    };
}
