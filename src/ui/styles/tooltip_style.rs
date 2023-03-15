use iced::widget::container;

pub struct TooltipContainer;
impl container::StyleSheet for TooltipContainer {
    type Style = iced::Theme;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        let palette = style.palette();
        let bg = palette.background;
        container::Appearance {
            text_color: Some(palette.text),
            background: Some(iced::Background::Color(iced::Color {
                r: bg.r,
                g: bg.g,
                b: bg.b,
                a: 1.,
            })),
            border_radius: 2.0,
            border_width: 0.0,
            border_color: palette.primary,
        }
    }
}

pub fn get_tooltip_style() -> iced::theme::Container {
    iced::theme::Container::Custom(Box::new(TooltipContainer))
}
