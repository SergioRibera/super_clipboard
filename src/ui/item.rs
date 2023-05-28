use iced::{
    alignment::Horizontal,
    widget::{container, image, text, Column},
    Element, Length,
};

use crate::settings::ClipboardItem;

use super::{mouse_listener::mouse_listener, styles::get_item_container_style, MainMessage};

#[must_use]
pub fn render_item(
    format_date: &str,
    i: usize,
    item: &ClipboardItem,
) -> Element<'static, MainMessage> {
    let fmt_date = item.format(format_date);
    let content: Element<MainMessage> = match item {
        ClipboardItem::Text(_, value) => {
            let mut col = Column::new().spacing(5.);
            if !format_date.is_empty() {
                col = col.push(
                    text(fmt_date)
                        .size(10.)
                        .width(Length::Fill)
                        .horizontal_alignment(Horizontal::Right),
                );
            }
            col.push(text(value.replace('\t', " ")).size(18.)).into()
        }
        ClipboardItem::Image(_, w, h, b) => {
            let mut col = Column::new().spacing(5.);
            if !format_date.is_empty() {
                col = col.push(
                    text(fmt_date)
                        .size(10.)
                        .width(Length::Fill)
                        .horizontal_alignment(Horizontal::Right),
                );
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
    mouse_listener(
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
