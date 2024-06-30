use iced::{Color, widget::{button, container, text_input}};
use iced::widget::text_input::Appearance;
use iced::Border;

#[derive(Debug, Clone, Copy)]
pub struct CustomContainerStyle;

impl container::StyleSheet for CustomContainerStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(Color::from_rgb8(43, 45, 49))),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CustomBackgroundStyle;

impl container::StyleSheet for CustomBackgroundStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(Color::from_rgb8(49, 51, 56))),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CustomButtonStyle;

impl button::StyleSheet for CustomButtonStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(Color::from_rgb8(35, 36, 40))),
            shadow_offset: iced::Vector::new(0.0, 0.0),
            text_color: Color::WHITE,
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(Color::from_rgb8(50, 52, 56))),
            shadow_offset: iced::Vector::new(0.0, 0.0),
            text_color: Color::WHITE,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CustomTextInputStyle;

impl text_input::StyleSheet for CustomTextInputStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            background: iced::Background::Color(Color::from_rgb8(56, 58, 64)),
            border: Border {
                color: Color::from_rgb8(120, 120, 120),
                width: 1.0,
                radius: [5.0, 5.0, 5.0, 5.0].into(),
            },
            icon_color: Color::from_rgb8(209, 212, 216),
        }
    }

    fn focused(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            background: iced::Background::Color(Color::from_rgb8(56, 58, 64)),
            border: Border {
                color: Color::from_rgb8(209, 212, 216),
                width: 1.0,
                radius: [5.0, 5.0, 5.0, 5.0].into(),
            },
            icon_color: Color::from_rgb8(209, 212, 216),
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgb8(209, 212, 216)
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgb8(209, 212, 216)
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgb8(72, 75, 80)
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgb8(120, 120, 120)
    }

    fn disabled(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            background: iced::Background::Color(Color::from_rgb8(56, 58, 64)),
            border: Border {
                color: Color::from_rgb8(120, 120, 120),
                width: 1.0,
                radius: [5.0, 5.0, 5.0, 5.0].into(),
            },
            icon_color: Color::from_rgb8(120, 120, 120),
        }
    }
}
