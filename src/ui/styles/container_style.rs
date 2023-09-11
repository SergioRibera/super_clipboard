use iced::widget::container;

pub struct ItemContainer;
impl container::StyleSheet for ItemContainer {
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
                a: bg.a + 0.025,
            })),
            border_radius: 0.0.into(),
            border_width: 0.0,
            border_color: palette.primary,
        }
    }
}

pub fn get_item_container_style() -> iced::theme::Container {
    iced::theme::Container::Custom(Box::new(ItemContainer))
}
