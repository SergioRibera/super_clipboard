use std::str::FromStr;

use device_query::DeviceQuery;
use global_hotkey::{hotkey::HotKey, GlobalHotKeyEvent};
use iced::widget::scrollable;
use iced::{window, Command};
use log::trace;

use crate::data::save;
use crate::ui::RouterView;
use crate::{
    settings::ClipboardItem,
    ui::{MainApp, MainMessage, SettingsModified},
    utils::{self, check_clipboard, track_mouse},
};

pub fn handle_update(app: &mut MainApp, message: MainMessage) -> Command<MainMessage> {
    match message {
        MainMessage::HiddeApplication => {
            #[cfg(not(debug_assertions))]
            {
                app.visible = false;
                app.follow = true;
                app.view = RouterView::Settings;
                window::change_mode(window::Mode::Hidden)
            }
            #[cfg(debug_assertions)]
            Command::none()
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
        MainMessage::TogglePinClipboard(index, item) => {
            if let Some(i) = index {
                app.pinned.remove_item(i);
            } else if let Some(item) = item {
                app.pinned.add_item(item.clone());
                if let Some(pos) = app.settings.clipboard().iter().position(|p| p == &item) {
                    app.settings.remove(pos);
                }
            }
            Command::none()
        }
        MainMessage::GeneratePassword => {
            app.clipboard_ctx
                .set_text(
                    app.password_generator
                        .generate_password(app.settings.password_generation().len),
                )
                .unwrap();
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
                        if let ClipboardItem::Text(_, s) = &item {
                            app.clipboard_ctx.set_text(s).unwrap();
                        }
                        app.settings.push(item);
                    }
                    utils::Message::RemoveLastClipboard => {
                        if !app.settings.clipboard().is_empty() {
                            app.settings.remove(app.settings.clipboard().len() - 1);
                        }
                    }
                }
            }
            Command::none()
        }
        MainMessage::CheckShortcuts(_) => {
            let mut commands = Vec::new();
            if app.follow {
                app.follow = false;
                let (x, y) =
                    track_mouse(app.last_data.mouse_pos, app.device_state.get_mouse().coords);
                app.last_data.mouse_pos = (x, y);
                commands.push(window::move_to(x, y));
            }

            if GlobalHotKeyEvent::receiver().try_recv().is_ok() {
                app.visible = true;
                app.view = RouterView::Home;

                commands.push(scrollable::scroll_to(
                    scrollable::Id::new("clipboard_scroll"),
                    scrollable::AbsoluteOffset { x: 0., y: 0. },
                ));
                commands.push(window::change_mode(window::Mode::Windowed));
            } else {
                app.follow = true;
                commands.clear();
            }

            Command::batch(commands)
        }
        MainMessage::CheckSettings(_) => {
            if app.pinned.is_changed {
                app.pinned.is_changed = false;
                save(&app.pinned, "pined");
            }
            if app.settings.is_changed {
                app.settings.is_changed = false;
                if !app.settings.store() {
                    let mut set = app.settings.clone();
                    set.set_clipboard(Vec::new());
                    save(&set, "data");
                } else {
                    save(&app.settings, "data");
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
                SettingsModified::ChangePassLen(v) => {
                    if let Ok(value) = v.parse::<usize>() {
                        app.settings.password_generation_mut().len = value;
                    }
                }
                SettingsModified::ChangePassUseSpecial(v) => {
                    app.settings.password_generation_mut().special = v
                }
                SettingsModified::ChangePassUseUpper(v) => {
                    app.settings.password_generation_mut().upper = v
                }
                SettingsModified::ChangePassUseLower(v) => {
                    app.settings.password_generation_mut().lower = v
                }
                SettingsModified::ChangePassUseNumber(v) => {
                    app.settings.password_generation_mut().number = v
                }
            }
            Command::none()
        }
    }
}
