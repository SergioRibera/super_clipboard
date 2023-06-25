use std::{
    sync::mpsc::{self, SyncSender},
    thread,
    time::Duration,
};

use arboard::Clipboard;
use device_query::{DeviceQuery, DeviceState};
use display_info::DisplayInfo;
use global_hotkey::GlobalHotKeyEvent;
use iced::{subscription, Subscription};
use log::{debug, info, trace};

use crate::{settings::ClipboardItem, APP_HEIGHT, APP_MOUSE_MARGIN, APP_WIDTH};

fn track_mouse(coords: (i32, i32), x: &mut i32, y: &mut i32, sender: SyncSender<Message>) {
    let (new_x, new_y) = coords;
    if *x != new_x || *y != new_y {
        *x = new_x;
        *y = new_y;
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
        sender
            .send(Message::ChangePosition((set_x, set_y)))
            .unwrap()
    }
    thread::sleep(Duration::from_millis(50));
}

fn check_clipboard(
    ctx: &mut Clipboard,
    last_str: &mut String,
    last_image: &mut (usize, usize, usize, [u8; 2]),
    preserve: bool,
    sender: SyncSender<Message>,
) {
    if let Ok(content) = ctx.get_text() {
        let is_empty = content.is_empty() || content == " ";
        if *last_str != content && !is_empty {
            info!("Content Received from clipboard: {content}");
            *last_str = content.clone();
            sender
                .send(Message::AddClipboard(ClipboardItem::from(content)))
                .unwrap();
        } else if *last_str != content && is_empty && !preserve {
            info!("Removing last item from clipboard");
            *last_str = content;
            sender.send(Message::RemoveLastClipboard).unwrap();
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
            sender
                .send(Message::AddClipboard(ClipboardItem::from(image)))
                .unwrap();
        }
    }
}

pub fn start_daemon(preserve: bool) -> Subscription<Event> {
    struct Daemon;
    trace!("Starting Instantiate daemon");

    subscription::unfold(
        std::any::TypeId::of::<Daemon>(),
        State::Disconnected(preserve),
        |state| async {
            match state {
                State::Disconnected(preserve) => {
                    let (sender, rec) = std::sync::mpsc::sync_channel::<Message>(20);
                    trace!("Creating mpsc channel");

                    thread::spawn(move || {
                        let mut ctx = Clipboard::new().unwrap();
                        let mut last_str = ctx.get_text().unwrap_or_default();
                        let mut last_image = (0usize, 0usize, 0usize, [0u8; 2]);
                        trace!("Clipboard context created");

                        let device_state = DeviceState::new();
                        let (mut x, mut y) = (0, 0);
                        trace!("Device Query instance created");
                        loop {
                            track_mouse(
                                device_state.get_mouse().coords,
                                &mut x,
                                &mut y,
                                sender.clone(),
                            );
                            check_clipboard(
                                &mut ctx,
                                &mut last_str,
                                &mut last_image,
                                preserve,
                                sender.clone(),
                            );
                            if let Ok(_) = GlobalHotKeyEvent::receiver().try_recv() {
                                sender.send(Message::ToggleVisibility(true)).unwrap();
                            }
                            thread::sleep(Duration::from_millis(200));
                        }
                    });

                    (Some(Event::Connected), State::Connected(rec))
                }
                State::Connected(conn) => match conn.try_recv() {
                    Ok(msg) => (Some(Event::Message(msg)), State::Connected(conn)),
                    Err(_) => (None, State::Connected(conn)),
                },
            }
        },
    )
}

#[derive(Debug)]
enum State {
    Disconnected(bool),
    Connected(mpsc::Receiver<Message>),
}

#[derive(Debug, Clone)]
pub enum Event {
    Connected,
    Message(Message),
}

#[derive(Debug, Clone)]
pub enum Message {
    RemoveLastClipboard,
    ToggleVisibility(bool),
    ChangePosition((i32, i32)),
    AddClipboard(ClipboardItem),
}
