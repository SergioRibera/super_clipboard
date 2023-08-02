use std::str::FromStr;

use device_query::DeviceQuery;
use global_hotkey::{hotkey::HotKey, GlobalHotKeyEvent};
use iced::{window, Command};
use iced_native::futures::SinkExt;
use iced_native::Executor;
use log::trace;

use crate::data::{save_pined, save_settings};
use crate::sync::{self, MDnsMessage};
use crate::ui::notify::accept_link_request;
use crate::{
    settings::ClipboardItem,
    ui::{MainApp, MainMessage, SettingsModified},
    utils::{self, check_clipboard, track_mouse},
};

fn send_to_devices(app: &MainApp, msg: MDnsMessage) {
    if let Some(sender) = app.sync_sender.clone() {
        let item = msg.clone();
        let mut sender = sender.clone();
        iced::executor::Default::new().unwrap().spawn(async move {
            sender.send(item).await.unwrap();
        });
    }
}

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
        MainMessage::SendLinkRequest(to) => {
            if let Some(device) = app.devices.iter_mut().find(|d| d.0 == to) {
                device.1 = true;
                send_to_devices(
                    app,
                    MDnsMessage::LinkRequest {
                        from: app.settings.device().clone(),
                        to,
                    },
                );
            }
            Command::none()
        }
        MainMessage::SyncDaemon(e) => {
            let Some(e) = e else { return Command::none(); };
            let my_device = app.settings.device();
            match e {
                sync::Event::Message(msg) => match msg {
                    sync::MDnsMessage::Connected(device) => {
                        if !app.settings.linked_devices().contains(&device)
                            && device.device_id != my_device.device_id
                        {
                            app.devices.push((device, false));
                        }
                    }
                    sync::MDnsMessage::Clipboard { device: _, item } => match item {
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
                    },
                    sync::MDnsMessage::LinkRequest { from, to: me } => {
                        if me.device_id == my_device.device_id {
                            accept_link_request(
                                &from.clone(),
                                "",
                                || {
                                    app.settings.add_linked_device(from.clone());
                                    // TODO: remove from list of devices
                                    send_to_devices(
                                        app,
                                        MDnsMessage::LinkAccepted { from: me, to: from },
                                    );
                                },
                                |_| {},
                            );
                        }
                    }
                    MDnsMessage::LinkAccepted {
                        from: device_accept_me,
                        to: me,
                    } => {
                        if me.device_id == my_device.device_id {
                            app.settings.add_linked_device(device_accept_me);
                        }
                    }
                    // TODO: add device to avialable device (app.devices) when detect new device
                    _ => {}
                },
                sync::Event::Connected(sender) => app.sync_sender = Some(sender),
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
                        app.settings.push(item.clone());
                        send_to_devices(
                            app,
                            MDnsMessage::Clipboard {
                                device: app.settings.device().clone(),
                                item,
                            },
                        )
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
            if app.pinned.is_changed {
                app.pinned.is_changed = false;
                save_pined(&app.pinned);
            }
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
                SettingsModified::ChangeDeviceName(v) => app.settings.set_device_name(v),
            }
            Command::none()
        }
    }
}
