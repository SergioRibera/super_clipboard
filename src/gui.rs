use std::sync::atomic::AtomicBool;

use iced::{
    widget::{svg, text, Row},
    Element, Length,
};

use crate::ui::MainMessage;

// if true listen to reset shortcut
pub static LISTEN_KEYBOARD: AtomicBool = AtomicBool::new(false);

fn tip(tip_icon: svg::Handle, msg: &str) -> impl Into<Element<'_, MainMessage>> {
    Row::new()
        .push(
            svg(tip_icon)
                .width(Length::Fixed(18.))
                .height(Length::Fixed(18.)),
        )
        .push(text(msg).size(15))
        .spacing(12.)
        .padding([0., 10.])
        .align_items(iced::Alignment::Center)
        .width(Length::Fill)
        .height(Length::Shrink)
}

pub mod home {
    use iced::{
        alignment::{Horizontal, Vertical},
        widget::{button, column, svg, text, tooltip, Row},
        Element, Length,
    };

    use crate::{
        settings::{ClipboardItem, ThemeType},
        ui::{
            item::render_item,
            styles::{get_btn_transparent_style, get_tooltip_style},
            MainMessage, RouterView,
        },
    };

    pub fn top_bar<'a>(
        theme: &ThemeType,
        dark_icon: svg::Handle,
        light_icon: svg::Handle,
        trash_icon: svg::Handle,
        settings_icon: svg::Handle,
    ) -> impl Into<Element<'a, MainMessage>> {
        let theme_icon = match theme {
            ThemeType::Light => dark_icon,
            ThemeType::Dark => light_icon,
        };

        Row::new()
            .push(
                text("Clipboard")
                    .size(20)
                    .width(Length::FillPortion(2))
                    .vertical_alignment(Vertical::Center)
                    .horizontal_alignment(Horizontal::Left),
            )
            .push(
                Row::new()
                    .push(
                        tooltip(
                            button(
                                svg(trash_icon)
                                    .width(Length::Fixed(18.))
                                    .height(Length::Fixed(18.)),
                            )
                            .padding(0)
                            .style(get_btn_transparent_style())
                            .on_press(MainMessage::ClearClipboard),
                            "Remove All",
                            tooltip::Position::Bottom,
                        )
                        .style(get_tooltip_style())
                        .size(15.),
                    )
                    .push(
                        tooltip(
                            button(
                                svg(theme_icon)
                                    .width(Length::Fixed(20.))
                                    .height(Length::Fixed(20.)),
                            )
                            .padding(0)
                            .style(get_btn_transparent_style())
                            .on_press(MainMessage::ThemeChangedToggle),
                            "Toggle Theme",
                            tooltip::Position::Bottom,
                        )
                        .style(get_tooltip_style())
                        .size(15.),
                    )
                    .push(
                        tooltip(
                            button(
                                svg(settings_icon)
                                    .width(Length::Fixed(15.))
                                    .height(Length::Fixed(15.)),
                            )
                            .padding(0)
                            .style(get_btn_transparent_style())
                            .on_press(MainMessage::ChangeView(RouterView::Settings)),
                            "Show Settings",
                            tooltip::Position::Bottom,
                        )
                        .style(get_tooltip_style())
                        .size(15.),
                    )
                    .spacing(5)
                    .align_items(iced::Alignment::Center)
                    .width(Length::Shrink)
                    .height(Length::Shrink),
            )
            .width(Length::Fill)
            .height(Length::Shrink)
            .align_items(iced::Alignment::Fill)
    }

    pub fn tip_section<'a>(tip_icon: svg::Handle) -> impl Into<Element<'a, MainMessage>> {
        super::tip(
            tip_icon,
            "Select an item in your clipboard history to paste it",
        )
    }

    pub fn show_items(
        fmt: &str,
        settings: &[ClipboardItem],
    ) -> impl Into<Element<'static, MainMessage>> {
        column(
            settings
                .iter()
                .enumerate()
                .rev()
                .map(|(i, item)| render_item(fmt, i, item))
                .collect(),
        )
        .width(Length::Fill)
        .spacing(5)
    }
}

pub mod settings {
    use iced::{
        widget::{button, checkbox, column, row, svg, text, text_input, tooltip, Column},
        Element, Length,
    };

    use crate::{
        settings::AppSettings,
        ui::{
            mouse_listener::mouse_listener,
            styles::{
                get_btn_transparent_style, get_input_keys_listening_style,
                get_input_keys_none_style, get_tooltip_style,
            },
            MainMessage, RouterView, SettingsModified,
        },
    };

    use super::LISTEN_KEYBOARD;

    pub fn back_bar<'a>(back_icon: svg::Handle) -> impl Into<Element<'a, MainMessage>> {
        button(
            row(vec![
                svg(back_icon)
                    .width(Length::Fixed(15.))
                    .height(Length::Fixed(15.))
                    .into(),
                text("Back").size(18.).into(),
            ])
            .spacing(10.),
        )
        .style(get_btn_transparent_style())
        .on_press(MainMessage::ChangeView(RouterView::Home))
    }

    pub fn tip_section<'a>(tip_icon: svg::Handle) -> impl Into<Element<'a, MainMessage>> {
        super::tip(tip_icon, "All changes requires end tick time to save")
    }

    pub fn list_options<'a>() -> impl Into<Element<'a, MainMessage>> {
        column(
            [
                ("Theme", "Set application theme", ""),
                (
                    "App Transparent",
                    "Toggle transparent background (require restart)",
                    "",
                ),
                ("Store Clipboard", "Save clipboard when close", ""),
                ("Preserve", "Preserve externally removed clipboard items. Usually removed by password managers", ""),
                ("Max Capacity", "Set max capacity for clipboard history", ""),
                (
                    "Tick to Save",
                    "Tick to save settings and store clipboard (in milliseconds)",
                    "",
                ),
                (
                    "Shortcut",
                    "Keys to active this window (click for modify)",
                    "modify_key",
                ),
                (
                    "Date format",
                    "Show date with this format (click for more information)",
                    "https://docs.rs/chrono/latest/chrono/format/strftime/index.html",
                ),
            ]
            .iter()
            .map(|(name, tip, url)| {
                tooltip(
                    mouse_listener(
                        text(name)
                            .size(20.)
                            .height(28.)
                            .horizontal_alignment(iced::alignment::Horizontal::Left)
                            .vertical_alignment(iced::alignment::Vertical::Center),
                    )
                    .on_press(MainMessage::Open(url.to_string())),
                    tip,
                    tooltip::Position::Bottom,
                )
                .style(get_tooltip_style())
                .size(15.)
                .into()
            })
            .collect(),
        )
        .spacing(10.)
        .width(Length::Shrink)
        .height(Length::Shrink)
    }

    pub fn list_elements(settings: &AppSettings) -> impl Into<Element<'_, MainMessage>> {
        let is_listening = LISTEN_KEYBOARD.load(std::sync::atomic::Ordering::SeqCst);
        let shortcut_style = if is_listening {
            get_input_keys_listening_style()
        } else {
            get_input_keys_none_style()
        };
        Column::new()
            .push(
                button(text(settings.get_theme().toggle_str()).size(20.))
                    .on_press(MainMessage::ThemeChangedToggle),
            )
            .push(checkbox("", settings.transparent(), |value| {
                MainMessage::ChangeSettings(SettingsModified::ChangeTransparency(value))
            }))
            .push(checkbox("", settings.store(), |value| {
                MainMessage::ChangeSettings(SettingsModified::StoreClipboard(value))
            }))
            .push(checkbox("", settings.preserve(), |value| {
                MainMessage::ChangeSettings(SettingsModified::StorePreserve(value))
            }))
            .push(text_input(
                "",
                &settings.max_capacity().to_string(),
                |value| MainMessage::ChangeSettings(SettingsModified::MaxCapacity(value)),
            ))
            .push(text_input("", &settings.tick_save().to_string(), |value| {
                MainMessage::ChangeSettings(SettingsModified::TickToSave(value))
            }))
            .push(
                text_input("", &settings.shortcut().join("+"), |value| {
                    MainMessage::ChangeSettings(SettingsModified::ChangeShortcut(value))
                })
                .style(shortcut_style),
            )
            .push(text_input("", settings.format_date(), |value| {
                MainMessage::ChangeSettings(SettingsModified::DateFormat(value))
            }))
            .spacing(12.)
            .width(Length::Shrink)
            .height(Length::Shrink)
    }
}
