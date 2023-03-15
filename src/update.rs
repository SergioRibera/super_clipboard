use iced::{
    keyboard::{KeyCode, Modifiers},
    window, Command,
};

use crate::{
    daemon,
    settings::{save_settings, ClipboardItem},
    ui::{MainApp, MainMessage, SettingsModified},
};

pub fn handle_update(app: &mut MainApp, message: MainMessage) -> Command<MainMessage> {
    match message {
        MainMessage::Open(url) => {
            if !url.is_empty() {
                open::that(url).unwrap_or_default();
            }
            Command::none()
        }
        MainMessage::ChangeView(view) => {
            app.view = view;
            Command::none()
        }
        MainMessage::ThemeChangedToggle => {
            app.settings.toggle_theme();
            Command::none()
        }
        MainMessage::ClearClipboard => {
            app.settings.clear();
            app.clipboard_ctx.clear().unwrap();
            Command::none()
        }
        MainMessage::RemoveClipboard(i) => {
            app.settings.remove(i);
            Command::none()
        }
        MainMessage::FollowMouse => {
            app.follow = true;
            Command::none()
        }
        MainMessage::CheckSettings(_) => {
            if app.settings.is_changed {
                app.settings.is_changed = false;
                if !app.settings.store() {
                    let mut set = app.settings.clone();
                    set.set_clipboard(Vec::new());
                    save_settings(&set);
                } else {
                    save_settings(&app.settings);
                }
            }
            Command::none()
        }
        MainMessage::SetClipboard(item) => {
            match item {
                ClipboardItem::Text(_, c) => app.clipboard_ctx.set_text(c).unwrap(),
                ClipboardItem::Image(_, width, height, bytes) => {
                    app.clipboard_ctx
                        .set_image(arboard::ImageData {
                            width,
                            height,
                            bytes: bytes.into(),
                        })
                        .unwrap();
                }
            }
            Command::none()
        }
        MainMessage::DaemonEvent(e) => match e {
            daemon::Event::Message(message) => match message {
                daemon::Message::ToggleVisibility(v) => {
                    app.visible = v;
                    app.follow = false;
                    if v {
                        window::change_mode(window::Mode::Windowed)
                    } else {
                        window::change_mode(window::Mode::Hidden)
                    }
                }
                daemon::Message::ChangePosition((x, y)) => {
                    if app.visible || app.follow {
                        window::move_to(x, y)
                    } else {
                        Command::none()
                    }
                }
                daemon::Message::AddClipboard(item) => {
                    app.settings.push(item);
                    Command::none()
                }
            },
            _ => Command::none(),
        },
        MainMessage::ChangeSettings(set) => {
            match set {
                SettingsModified::MaxCapacity(v) => {
                    if let Ok(value) = v.parse::<u64>() {
                        app.settings.set_max_capacity(value);
                    }
                }
                SettingsModified::TickToSave(v) => {
                    if let Ok(value) = v.parse::<u64>() {
                        app.settings.set_tick_save(value);
                    }
                }
                SettingsModified::StoreClipboard(v) => app.settings.set_store(v),
                SettingsModified::DateFormat(v) => app.settings.set_format_date(v),
                SettingsModified::ChangeTransparency(v) => app.settings.set_transparent(v),
            }
            Command::none()
        }
    }
}