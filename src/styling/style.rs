use super::theme::Theme;

use iced::{
    application,
    widget::{button, container, svg, text},
    Background,
};

#[derive(Default, Debug, Clone, Copy)]
pub enum Application {
    #[default]
    Default,
}

impl application::StyleSheet for Theme {
    type Style = Application;

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: self.palette().background,
            text_color: self.palette().text,
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum Container {
    #[default]
    Default,
    Bordered,
}

impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        match style {
            Container::Default => container::Appearance {
                background: Some(Background::Color(self.palette().background)),
                text_color: Some(self.palette().text),
                ..Default::default()
            },
            Container::Bordered => container::Appearance {
                background: Some(Background::Color(self.palette().background)),
                text_color: Some(self.palette().text),
                border: self.palette().border,
                ..Default::default()
            },
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum Text {
    #[default]
    Paragraph,
}

impl text::StyleSheet for Theme {
    type Style = Text;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        match style {
            Text::Paragraph => text::Appearance {
                color: Some(self.palette().text),
            },
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum Svg {
    #[default]
    Default,
    Icon,
}

impl svg::StyleSheet for Theme {
    type Style = Svg;

    fn appearance(&self, style: &Self::Style) -> svg::Appearance {
        match style {
            Svg::Default => svg::Appearance { color: None },
            Svg::Icon => svg::Appearance {
                color: Some(self.palette().text),
            },
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum Button {
    #[default]
    Default,
    Circular,
}

impl button::StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        match style {
            Button::Default => button::Appearance {
                background: Some(iced::Background::Color(self.palette().background)),
                text_color: self.palette().text,
                border: self.palette().border,
                ..Default::default()
            },
            Button::Circular => button::Appearance {
                background: Some(iced::Background::Color(self.palette().background)),
                text_color: self.palette().text,
                border: iced::Border {
                    color: self.palette().secondary,
                    width: 2.0,
                    radius: iced::border::Radius::from(75/2),
                },
                ..Default::default()
            },
        }
    }
}
