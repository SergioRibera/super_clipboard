use arboard::Clipboard;
use display_info::DisplayInfo;
use log::info;

use crate::{settings::ClipboardItem, APP_HEIGHT, APP_MOUSE_MARGIN, APP_WIDTH};

#[derive(Debug, Clone)]
pub enum Message {
    RemoveLastClipboard,
    AddClipboard(ClipboardItem),
}

pub fn track_mouse(old: (i32, i32), coords: (i32, i32)) -> (i32, i32) {
    let (old_x, old_y) = old;
    let (x, y) = coords;
    let Ok(DisplayInfo {
        width,
        height,
        x: display_x,
        y: display_y,
        ..
    }) = DisplayInfo::from_point(x, y)
    else {
        return coords;
    };

    if old_x == x || old_y == y {
        return coords;
    }

    let left_x = ((x - display_x) as u32) < (width / 2);
    let bottom_y = ((y - display_y) as u32) > (height / 2);

    match (left_x, bottom_y) {
        // Top Left
        (true, false) => (x - APP_MOUSE_MARGIN, y - APP_MOUSE_MARGIN),
        // Top Right
        (false, false) => ((x - APP_WIDTH) + APP_MOUSE_MARGIN, y - APP_MOUSE_MARGIN),
        // Bottom Left
        (true, true) => (x - APP_MOUSE_MARGIN, y - APP_HEIGHT + APP_MOUSE_MARGIN),
        // Bottom Right
        (false, true) => (
            (x - APP_WIDTH) + APP_MOUSE_MARGIN,
            y - APP_HEIGHT + APP_MOUSE_MARGIN,
        ),
    }
}

pub fn check_clipboard(
    ctx: &mut Clipboard,
    last_str: &mut String,
    last_image: &mut (usize, usize, usize, [u8; 2]),
    preserve: bool,
) -> Option<Message> {
    if let Ok(content) = ctx.get_text() {
        let is_empty = content.is_empty() || content == " ";
        if *last_str != content && !is_empty {
            info!("Content Received from clipboard: {content}");
            *last_str = content.clone();
            return Some(Message::AddClipboard(ClipboardItem::from(content)));
        } else if *last_str != content && is_empty && !preserve {
            info!("Removing last item from clipboard");
            *last_str = content;
            return Some(Message::RemoveLastClipboard);
        }
    }
    if let Ok(image) = ctx.get_image() {
        let (l_w, l_h, l_l, bs) = last_image;
        if !image.bytes.is_empty()
            && image.width != *l_w
            && image.height != *l_h
            && image.bytes.len() != *l_l
            && image.bytes.first().unwrap() != &bs[0]
            && image.bytes.last().unwrap() != &bs[1]
        {
            last_image.0 = image.width;
            last_image.1 = image.height;
            last_image.2 = image.bytes.len();
            last_image.3[0] = *image.bytes.first().unwrap();
            last_image.3[1] = *image.bytes.last().unwrap();
            info!("Image Received from clipboard: {image:?}");
            return Some(Message::AddClipboard(ClipboardItem::from(image)));
        }
    }
    None
}
