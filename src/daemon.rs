use std::{
    sync::mpsc::{self, SyncSender},
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

fn track_mouse(coords: (i32, i32), h: i32, x: &mut i32, y: &mut i32, sender: SyncSender<Message>) {
    let (new_x, new_y) = coords;
    if *x != new_x || *y != new_y {
        *x = new_x;
        *y = new_y;
        // TODO: ideally here check if mouse is not hover window
        let set_x = *x - (APP_WIDTH / 2);
        let set_y = if *y > h / 2 {
            *y - APP_HEIGHT + APP_MOUSE_MARGIN
        } else {
            *y - APP_MOUSE_MARGIN
        };
        debug!("New mouse position: {set_x}x{set_y}");
        trace!("Sending new mouse position");
        sender
            .send(Message::ChangePosition((set_x, set_y)))
            .unwrap()
    }
    thread::sleep(Duration::from_millis(50));
}

fn listen_keyboard(keys: Vec<Keycode>, shortcuts: Vec<Keycode>, sender: SyncSender<Message>) {
    if keys.is_empty() || keys.len() < 2 {
        return;
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
            return;
        }
        trace!("Sending new keys presseds");
        sender
            .send(Message::ChangeKeys(
                keys.iter().map(|k| keycode_to_str(k)).collect(),
            ))
            .unwrap();
    }
}

fn check_clipboard(ctx: &mut Clipboard, last_str: &mut String, sender: SyncSender<Message>) {
    if let Ok(content) = ctx.get_text() {
        if *last_str != content && !content.is_empty() {
            info!("Content Received from clipboard: {content}");
            *last_str = content.clone();
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

                    thread::spawn(move || {
                        let mut ctx = Clipboard::new().unwrap();
                        let mut last_str = ctx.get_text().unwrap_or_default();
                        trace!("Clipboard context created");

                        let device_state = DeviceState::new();
                        let (mut x, mut y) = (0, 0);
                        trace!("Device Query instance created");
                        let h = Enigo::new().main_display_size().1;
                        loop {
                            track_mouse(
                                device_state.get_mouse().coords,
                                h,
                                &mut x,
                                &mut y,
                                sender.clone(),
                            );
                            listen_keyboard(
                                device_state.get_keys(),
                                shortcuts.clone(),
                                sender.clone(),
                            );
                            check_clipboard(&mut ctx, &mut last_str, sender.clone());
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
