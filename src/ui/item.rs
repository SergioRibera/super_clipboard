use iced::widget::{button, svg, tooltip, Row};
use iced::{
    alignment::Horizontal,
    widget::{container, image, mouse_area, text, Column},
    Element, Length,
};

use crate::settings::ClipboardItem;

use super::styles::{get_btn_transparent_style, get_tooltip_style};
use super::{styles::get_item_container_style, MainMessage};

#[must_use]
pub fn render_item(
    format_date: &str,
    i: usize,
    i_pinned: Option<usize>,
    item: &ClipboardItem,
    pin_icon: svg::Handle,
    unpin_icon: svg::Handle,
) -> Element<'static, MainMessage> {
    let fmt_date = item.format(format_date);
    let content: Element<MainMessage> = match item {
        ClipboardItem::Text(_, value) => {
            let mut col = Column::new().spacing(5.);
            if !format_date.is_empty() {
                col = col.push(top_item(fmt_date, i_pinned, item, pin_icon, unpin_icon));
            }
            col.push(text(value.replace('\t', " ")).size(18.)).into()
        }
        ClipboardItem::Image(_, w, h, b) => {
            let mut col = Column::new().spacing(5.);
            if !format_date.is_empty() {
                col = col.push(top_item(fmt_date, i_pinned, item, pin_icon, unpin_icon));
            }
            let bytes = b.clone();
            col.push(
                image(image::Handle::from_pixels(*w as u32, *h as u32, bytes))
                    .content_fit(iced::ContentFit::Contain)
                    .width(Length::Fixed(40.))
                    .height(Length::Fixed(40.)),
            )
            .into()
        }
    };
    mouse_area(
        container(content)
            .width(Length::Fill)
            .height(Length::Shrink)
            .padding(10.)
            .style(get_item_container_style()),
    )
    .on_press(MainMessage::SetClipboard(item.clone()))
    .on_right_press(MainMessage::RemoveClipboard(i))
    .into()
}

fn top_item(
    fmt_date: String,
    i_pinned: Option<usize>,
    item: &ClipboardItem,
    pin_icon: svg::Handle,
    unpin_icon: svg::Handle,
) -> Element<'static, MainMessage> {
    let mut row = Row::new().align_items(iced::Alignment::Center);
    log::trace!("Index Pinned: {i_pinned:?}");
    let icon = i_pinned.as_ref().map(|_| unpin_icon).unwrap_or(pin_icon);
    let tooltip_text = i_pinned
        .as_ref()
        .map(|_| "Unpin Item")
        .unwrap_or("Pin Item");

    row = row.push(
        tooltip(
            button(
                svg(icon)
                    .width(Length::Fixed(15.0))
                    .height(Length::Fixed(15.0)),
            )
            .padding(0)
            .style(get_btn_transparent_style())
            .on_press(MainMessage::TogglePinClipboard(
                i_pinned,
                Some(item.clone()),
            )),
            tooltip_text,
            tooltip::Position::Right,
        )
        .style(get_tooltip_style())
        .size(15.),
    );
    row = row.push(
        text(fmt_date)
            .size(10.)
            .width(Length::Fill)
            .horizontal_alignment(Horizontal::Right),
    );
    row.into()
}
