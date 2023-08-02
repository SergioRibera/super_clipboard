use iced::widget::{button, container, svg, text, Column, Row};
use iced::Element;
use iced_native::svg::Handle;
use iced_native::Length;

use crate::sync::MDnsDevice;

// use super::styles::{get_btn_transparent_style, get_tooltip_style};
use super::{styles::get_item_container_style, MainMessage};

#[must_use]
pub fn render_device(args: (usize, &(MDnsDevice, bool), bool)) -> Element<'static, MainMessage> {
    let (_i, (device, linked_req_sended), aviable_send_link_req) = args;
    let mut content = Column::new().push(
        Row::new()
            .push(svg(get_os_icon(device.os.as_str())).width(Length::Fixed(15.)))
            .push(
                text(device.name.clone())
                    .width(Length::Fill)
                    .horizontal_alignment(iced_native::alignment::Horizontal::Left),
            )
            .spacing(10)
            .align_items(iced_native::Alignment::Center)
            .width(Length::Fill)
            .height(Length::Shrink),
    );

    if aviable_send_link_req {
        let mut btn = button("Send Link Request").padding([5, 10]);
        if !linked_req_sended {
            btn = btn.on_press(MainMessage::SendLinkRequest(device.clone()));
        }
        content = content.push(btn);
    }

    container(
        content
            .width(Length::Fill)
            .spacing(10)
            .align_items(iced_native::Alignment::End),
    )
    .width(Length::Fill)
    .height(Length::Shrink)
    .padding(10)
    .style(get_item_container_style())
    .into()
}

fn get_os_icon(os: &str) -> Handle {
    match os {
        "linux" => Handle::from_memory(include_bytes!("../../assets/linux.svg").to_vec()),
        "macos" | "ios" => Handle::from_memory(include_bytes!("../../assets/apple.svg").to_vec()),
        "windows" => Handle::from_memory(include_bytes!("../../assets/windows.svg").to_vec()),
        "android" => Handle::from_memory(include_bytes!("../../assets/android.svg").to_vec()),
        _ => Handle::from_memory(include_bytes!("../../assets/generic.svg").to_vec()),
    }
}
