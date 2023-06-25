use iced::widget::text_input;

pub struct TextInputStyle(bool);

impl text_input::StyleSheet for TextInputStyle {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        let palette = style.extended_palette();

        text_input::Appearance {
            background: palette.background.base.color.into(),
            border_radius: 2.0,
            border_width: 1.0,
            border_color: palette.background.strong.color,
            icon_color: palette.background.strong.color,
        }
    }

    fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
        let palette = style.extended_palette();

        text_input::Appearance {
            background: palette.background.base.color.into(),
            border_radius: 2.0,
            border_width: 1.0,
            border_color: palette.background.base.text,
            icon_color: palette.background.base.text,
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        let palette = style.extended_palette();

        text_input::Appearance {
            background: palette.background.base.color.into(),
            border_radius: 2.0,
            border_width: 1.0,
            border_color: palette.primary.strong.color,
            icon_color: palette.primary.strong.color,
        }
    }

    fn placeholder_color(&self, style: &Self::Style) -> iced::Color {
        let palette = style.extended_palette();

        palette.background.strong.color
    }

    fn value_color(&self, style: &Self::Style) -> iced::Color {
        let palette = style.extended_palette();

        if self.0 {
            palette.primary.base.color
        } else {
            palette.background.base.text
        }
    }

    fn selection_color(&self, style: &Self::Style) -> iced::Color {
        let palette = style.extended_palette();

        palette.primary.weak.color
    }

    fn disabled_color(&self, style: &Self::Style) -> iced_native::Color {
        let palette = style.extended_palette();

        let [r, g, b, a] = palette.primary.base.text.into_linear();
        iced_native::Color::new(r, g, b, a / 2.)
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        let palette = style.extended_palette();

        text_input::Appearance {
            background: palette.background.base.color.into(),
            border_radius: 2.0,
            border_width: 1.0,
            border_color: palette.background.base.text,
            icon_color: palette.background.base.text,
        }
    }
}

#[allow(dead_code)]
pub fn get_input_keys_listening_style() -> iced::theme::TextInput {
    iced::theme::TextInput::Custom(Box::new(TextInputStyle(true)))
}

pub fn get_input_keys_none_style() -> iced::theme::TextInput {
    iced::theme::TextInput::Custom(Box::new(TextInputStyle(false)))
}
