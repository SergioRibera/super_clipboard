use iced::widget::{Button, Row};
use iced::{
    alignment::Horizontal,
    widget::{container, image, mouse_area, text, Column},
    Element, Length,
};

use crate::settings::ClipboardItem;

use super::{styles::get_item_container_style, MainMessage};

#[must_use]
pub fn render_item(
    format_date: &str,
    i: usize,
    i_pinned: Option<usize>,
    item: &ClipboardItem,
) -> Element<'static, MainMessage> {
    let fmt_date = item.format(format_date);
    let content: Element<MainMessage> = match item {
        ClipboardItem::Text(_, value) => {
            let mut col = Column::new().spacing(5.);
            if !format_date.is_empty() {
                col = col.push(top_item(fmt_date, i_pinned, item));
            }
            col.push(text(value.replace('\t', " ")).size(18.)).into()
        }
        ClipboardItem::Image(_, w, h, b) => {
            let mut col = Column::new().spacing(5.);
            if !format_date.is_empty() {
                col = col.push(top_item(fmt_date, i_pinned, item));
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
) -> Element<'static, MainMessage> {
    let mut row = Row::new();
    let content = i_pinned.map(|_| "Remove").or(Some("Add")).unwrap();
    row = row.push(
        Button::new(content)
            .on_press(MainMessage::TogglePinClipboard(
                i_pinned,
                Some(item.clone()),
            ))
            .width(Length::Fixed(24.0)),
    );
    row = row.push(
        text(fmt_date)
            .size(10.)
            .width(Length::Fill)
            .horizontal_alignment(Horizontal::Right),
    );
    row.into()
}
