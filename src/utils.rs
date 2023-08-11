use display_info::DisplayInfo;
use log::{debug, info, trace};
use shared::{arboard::Clipboard, clipboard::ClipboardItem};

use crate::{APP_HEIGHT, APP_MOUSE_MARGIN, APP_WIDTH};

#[derive(Debug, Clone)]
pub enum Message {
    RemoveLastClipboard,
    AddClipboard(ClipboardItem),
}

pub fn track_mouse(old: (i32, i32), coords: (i32, i32)) -> (i32, i32) {
    let (old_x, old_y) = old;
    let (new_x, new_y) = coords;
    if old_x != new_x || old_y != new_y {
        // TODO: ideally here check if mouse is not hover window
        let (set_x, set_y) = if let Ok(display_info) = DisplayInfo::from_point(new_x, new_y) {
            debug!("Device Information based on point ({new_x}x{new_y}): {display_info:?}");
            let x = if new_x
                > (display_info.y + display_info.width as i32) - APP_WIDTH - APP_MOUSE_MARGIN
            {
                // right limit
                new_x - (APP_WIDTH - APP_MOUSE_MARGIN)
            } else if new_x < display_info.x + APP_WIDTH + APP_MOUSE_MARGIN {
                // Left
                new_x
            } else {
                // center or default
                new_x - (APP_WIDTH / 2)
            };
            let y =
                if new_y > ((display_info.y + display_info.height as i32) / 2) - APP_MOUSE_MARGIN {
                    // top
                    new_y - APP_HEIGHT + APP_MOUSE_MARGIN
                } else {
                    // bottom
                    new_y - APP_MOUSE_MARGIN
                };
            (x, y)
        } else {
            (new_x - (APP_WIDTH / 2), new_y - APP_MOUSE_MARGIN)
        };
        debug!("New mouse position: {set_x}x{set_y}");
        trace!("Sending new mouse position");
        return (set_x, set_y);
    }
    coords
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
