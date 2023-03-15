use std::{
    sync::mpsc::{self, Sender},
    thread,
    time::Duration,
};

use arboard::Clipboard;
use enigo::{Enigo, MouseControllable};
use iced::{subscription, Subscription};

use crate::{settings::ClipboardItem, APP_HEIGHT, APP_MOUSE_MARGIN, APP_WIDTH};

fn track_mouse(sender: Sender<Message>) {
    thread::spawn(move || {
        let e = Enigo::new();
        let (mut x, mut y) = e.mouse_location();
        loop {
            let (new_x, new_y) = e.mouse_location();
            if x != new_x || y != new_y {
                let (_w, h) = e.main_display_size();
                x = new_x;
                y = new_y;
                // TODO: ideally here check if mouse is not hover window
                let set_x = x - (APP_WIDTH / 2);
                let set_y = if y > h / 2 {
                    y - APP_HEIGHT - APP_MOUSE_MARGIN
                } else {
                    y + APP_MOUSE_MARGIN
                };
                sender
                    .send(Message::ChangePosition((set_x, set_y)))
                    .unwrap()
            }
            thread::sleep(Duration::from_millis(50));
        }
    });
}

fn check_clipboard(sender: Sender<Message>) {
    let mut ctx = Clipboard::new().unwrap();
    let mut last_str = ctx.get_text().unwrap_or_default();

    thread::spawn(move || loop {
        if let Ok(content) = ctx.get_text() {
            if last_str != content && !content.is_empty() {
                println!("Content Received: {content}");
                last_str = content.clone();
                sender
                    .send(Message::AddClipboard(ClipboardItem::from(content)))
                    .unwrap();
            }
        }
        if let Ok(image) = ctx.get_image() {
            if !image.bytes.is_empty() {
                println!("Image Received: {image:?}");
                sender
                    .send(Message::AddClipboard(ClipboardItem::from(image)))
                    .unwrap();
            }
        }
        thread::sleep(Duration::from_millis(200));
    });
}

pub fn start_daemon() -> Subscription<Event> {
    struct Daemon;

    subscription::unfold(
        std::any::TypeId::of::<Daemon>(),
        State::Disconnected,
        |state| async move {
            match state {
                State::Disconnected => {
                    let (sender, rec) = std::sync::mpsc::channel::<Message>();

                    track_mouse(sender.clone());
                    check_clipboard(sender);
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
    Disconnected,
    Connected(mpsc::Receiver<Message>),
}

#[derive(Debug, Clone)]
pub enum Event {
    Connected,
    Message(Message),
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleVisibility(bool),
    ChangePosition((i32, i32)),
    AddClipboard(ClipboardItem),
}
