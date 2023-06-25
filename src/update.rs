use std::str::FromStr;

use device_query::DeviceQuery;
use global_hotkey::{hotkey::HotKey, GlobalHotKeyEvent};
use iced::{window, Command};
use log::trace;

use crate::{
    settings::{save_settings, ClipboardItem},
    ui::{MainApp, MainMessage, SettingsModified},
    utils::{self, check_clipboard, track_mouse},
};

pub fn handle_update(app: &mut MainApp, message: MainMessage) -> Command<MainMessage> {
    match message {
        MainMessage::HiddeApplication => {
            app.visible = false;
            app.follow = true;
            window::change_mode(window::Mode::Hidden)
        }
        MainMessage::Open(url) => {
            if url.is_empty() {
                return Command::none();
            }
            open::that(url).unwrap_or_default();
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
        MainMessage::CheckClipboard(_) => {
            if let Some(msg) = check_clipboard(
                &mut app.clipboard_ctx,
                &mut app.last_data.last_str,
                &mut app.last_data.last_image,
                app.settings.preserve(),
            ) {
                match msg {
                    utils::Message::AddClipboard(item) => {
                        app.settings.push(item);
                    }
                    utils::Message::RemoveLastClipboard => {
                        app.settings.remove(app.settings.clipboard().len() - 1);
                    }
                }
            }
            Command::none()
        }
        MainMessage::CheckShortcuts(_) => {
            let mut commands = Vec::new();
            if GlobalHotKeyEvent::receiver().try_recv().is_ok() {
                app.visible = true;
                commands.push(window::change_mode(window::Mode::Windowed));
            }

            if app.follow && !commands.is_empty() {
                app.follow = false;
                let (x, y) =
                    track_mouse(app.last_data.mouse_pos, app.device_state.get_mouse().coords);
                app.last_data.mouse_pos = (x, y);
                commands.push(window::move_to(x, y));
            }
            Command::batch(commands)
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
                SettingsModified::StorePreserve(v) => app.settings.set_preserve(v),
                SettingsModified::DateFormat(v) => app.settings.set_format_date(v),
                SettingsModified::ChangeTransparency(v) => app.settings.set_transparent(v),
                SettingsModified::ChangeShortcut(v) => {
                    trace!("Settigns Change Keys");
                    if let Ok(hotkey) = HotKey::from_str(&v) {
                        app.hotkey = hotkey;
                        app.hotkeys_manager.unregister(app.hotkey).unwrap();
                        app.hotkeys_manager.register(app.hotkey).unwrap();
                    }
                    app.settings.set_shortcut(v)
                }
            }
            Command::none()
        }
    }
}
