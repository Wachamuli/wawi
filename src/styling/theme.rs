use iced::Color;

#[derive(Default, Debug, PartialEq, Eq, Copy, Clone)]
pub enum Theme {
    #[default]
    Dark,
    #[allow(unused)]
    Light,
}

#[allow(dead_code)]
pub struct Palette {
    pub background: Color,
    pub foreground: Color,
    pub primary: Color,
    pub secondary: Color,
    pub tertiary: Color,
    pub accent: Color,
    pub text: Color,
}

impl Theme {
    pub const BORDER_WIDTH: f32 = 2.0;

    pub fn palette(self) -> Palette {
        match self {
            Theme::Light => todo!(),
            Theme::Dark => Palette {
                background: Color::BLACK,
                foreground: Color::WHITE,
                primary: Color::BLACK,
                secondary: Color::WHITE,
                tertiary: Color::WHITE,
                accent: Color::WHITE,
                text: Color::WHITE,
            },
        }
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "Morpheus")
    }
}
