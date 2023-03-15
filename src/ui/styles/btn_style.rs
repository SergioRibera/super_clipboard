use iced::widget::button;

pub struct TransparentButton;
impl button::StyleSheet for TransparentButton {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            shadow_offset: iced::Vector::default(),
            background: None,
            border_radius: 0.,
            border_width: 0.,
            border_color: style.palette().primary,
            text_color: style.palette().text,
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        button::Appearance {
            shadow_offset: active.shadow_offset + iced::Vector::new(0.0, 1.0),
            ..active
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            shadow_offset: iced::Vector::default(),
            ..self.active(style)
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        button::Appearance {
            shadow_offset: iced::Vector::default(),
            text_color: iced::Color {
                a: active.text_color.a * 0.5,
                ..active.text_color
            },
            ..active
        }
    }
}

pub fn get_btn_transparent_style() -> iced::theme::Button {
    iced::theme::Button::Custom(Box::new(TransparentButton))
}
