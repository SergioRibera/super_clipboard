use std::{
    sync::{
        mpsc::{self, SyncSender},
        Weak,
    },
    thread,
    time::Duration,
};

use arboard::Clipboard;
use device_query::{DeviceQuery, DeviceState, Keycode};
use enigo::{Enigo, MouseControllable};
use iced::{subscription, Subscription};
use log::{debug, info, trace};

use crate::{
    gui::LISTEN_KEYBOARD,
    keys::{keycode_from_str, keycode_to_str},
    settings::ClipboardItem,
    APP_HEIGHT, APP_MOUSE_MARGIN, APP_WIDTH,
};

fn track_mouse(sender: SyncSender<Message>) {
    thread::spawn(move || {
        let device_state: Weak<DeviceState> = Weak::new();
        let (mut x, mut y) = (0, 0);
        trace!("Device Query instance created");
        let (_w, h) = Enigo::new().main_display_size();
        while let Some(e) = device_state.upgrade() {
            let (new_x, new_y) = e.query_pointer().coords;
            if x != new_x || y != new_y {
                x = new_x;
                y = new_y;
                // TODO: ideally here check if mouse is not hover window
                let set_x = x - (APP_WIDTH / 2);
                let set_y = if y > h / 2 {
                    y - APP_HEIGHT + APP_MOUSE_MARGIN
                } else {
                    y - APP_MOUSE_MARGIN
                };
                debug!("New mouse position: {set_x}x{set_y}");
                trace!("Sending new mouse position");
                sender
                    .send(Message::ChangePosition((set_x, set_y)))
                    .unwrap()
            }
            thread::sleep(Duration::from_millis(50));
        }
    });
}

fn listen_keyboard(shortcuts: Vec<Keycode>, sender: SyncSender<Message>) {
    thread::spawn(move || {
        let device_state: Weak<DeviceState> = Weak::new();
        while let Some(device_state) = device_state.upgrade() {
            let keys = device_state.get_keys();
            if keys.is_empty() || keys.len() < 2 {
                continue;
            }

            trace!("Get AtomicBool Ordering::SeqCst; LISTEN_KEYBOARD");
            let is_listening = LISTEN_KEYBOARD.load(std::sync::atomic::Ordering::SeqCst);

            if !is_listening && keys == shortcuts {
                info!("Shortcut pressed; Showing window; Set true to LISTEN_KEYBOARD");
                sender.send(Message::ToggleVisibility(true)).unwrap()
            } else {
                if keys.contains(&Keycode::Escape) {
                    info!("Set false to LISTEN_KEYBOARD");
                    LISTEN_KEYBOARD.store(false, std::sync::atomic::Ordering::SeqCst);
                    continue;
                }
                trace!("Sending new keys presseds");
                sender
                    .send(Message::ChangeKeys(
                        keys.iter().map(|k| keycode_to_str(k)).collect(),
                    ))
                    .unwrap();
            }
            thread::sleep(Duration::from_millis(50));
        }
    });
}

fn check_clipboard(sender: SyncSender<Message>) {
    let mut ctx = Clipboard::new().unwrap();
    let mut last_str = ctx.get_text().unwrap_or_default();
    trace!("Clipboard context created");

    thread::spawn(move || loop {
        if let Ok(content) = ctx.get_text() {
            if last_str != content && !content.is_empty() {
                info!("Content Received from clipboard: {content}");
                last_str = content.clone();
                sender
                    .send(Message::AddClipboard(ClipboardItem::from(content)))
                    .unwrap();
            }
        }
        if let Ok(image) = ctx.get_image() {
            if !image.bytes.is_empty() {
                info!("Image Received from clipboard: {image:?}");
                sender
                    .send(Message::AddClipboard(ClipboardItem::from(image)))
                    .unwrap();
            }
        }
        thread::sleep(Duration::from_millis(200));
    });
}

pub fn start_daemon(shortcuts: &[String]) -> Subscription<Event> {
    struct Daemon;
    trace!("Starting Instantiate daemon");

    let shortcuts = shortcuts
        .iter()
        .map(|k| keycode_from_str(k))
        .collect::<Vec<Keycode>>();

    info!("Application Shortuct mapped");

    subscription::unfold(
        std::any::TypeId::of::<Daemon>(),
        State::Disconnected(shortcuts),
        |state| async {
            match state {
                State::Disconnected(shortcuts) => {
                    let (sender, rec) = std::sync::mpsc::sync_channel::<Message>(20);
                    trace!("Creating mpsc channel");

                    track_mouse(sender.clone());
                    listen_keyboard(shortcuts, sender.clone());
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
    Disconnected(Vec<Keycode>),
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
    ChangeKeys(Vec<String>),
}
