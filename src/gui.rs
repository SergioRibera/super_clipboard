use iced::{
    widget::{svg, text, Row},
    Element, Length,
};

use crate::ui::MainMessage;

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
        passwd_icon: svg::Handle,
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
                                svg(passwd_icon)
                                    .width(Length::Fixed(18.))
                                    .height(Length::Fixed(18.)),
                            )
                            .padding(0)
                            .style(get_btn_transparent_style())
                            .on_press(MainMessage::GeneratePassword),
                            "Generate Password",
                            tooltip::Position::Bottom,
                        )
                        .style(get_tooltip_style())
                        .size(15.),
                    )
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
                            .on_press(MainMessage::ChangeView(RouterView::Settings(false))),
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
        pinned: &[ClipboardItem],
        pin_icon: svg::Handle,
        unpin_icon: svg::Handle,
    ) -> impl Into<Element<'static, MainMessage>> {
        let mut clip = pinned
            .iter()
            .enumerate()
            .rev()
            .map(|(i, item)| {
                render_item(fmt, i, Some(i), item, pin_icon.clone(), unpin_icon.clone())
            })
            .collect::<Vec<_>>();
        clip.extend(settings.iter().enumerate().rev().map(|(i, item)| {
            render_item(fmt, i, None, item, pin_icon.clone(), unpin_icon.clone())
        }));
        column(clip).width(Length::Fill).spacing(5)
    }
}

pub mod settings {
    use iced::widget::Row;
    use iced::{
        widget::{
            button, checkbox, column, mouse_area, row, svg, text, text_input, tooltip, Column,
        },
        Element, Length,
    };

    use crate::sync::MDnsDevice;
    use crate::ui::device::render_device;
    use crate::{
        settings::AppSettings,
        ui::{
            styles::{get_btn_transparent_style, get_input_keys_none_style, get_tooltip_style},
            MainMessage, RouterView, SettingsModified,
        },
    };

    pub fn back_bar<'a>(back_icon: svg::Handle) -> impl Into<Element<'a, MainMessage>> {
        Row::new()
            .push(
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
                .width(Length::FillPortion(2)),
            )
            .push(
                Row::new()
                    .push(
                        tooltip(
                            button(
                                "Ajustes", // svg(passwd_icon)
                                          //     .width(Length::Fixed(18.))
                                          //     .height(Length::Fixed(18.)),
                            )
                            .padding(0)
                            .style(get_btn_transparent_style())
                            .on_press(MainMessage::ChangeView(RouterView::Settings(false))),
                            "Settings Tab",
                            tooltip::Position::Bottom,
                        )
                        .style(get_tooltip_style())
                        .size(15.),
                    )
                    .push(
                        tooltip(
                            button(
                                "Devices", // svg(trash_icon)
                                          //     .width(Length::Fixed(18.))
                                          //     .height(Length::Fixed(18.)),
                            )
                            .padding(0)
                            .style(get_btn_transparent_style())
                            .on_press(MainMessage::ChangeView(RouterView::Settings(true))),
                            "Sync Devices Tab",
                            tooltip::Position::Bottom,
                        )
                        .style(get_tooltip_style())
                        .size(15.),
                    )
                    .spacing(15)
                    .align_items(iced::Alignment::Center)
                    .width(Length::Shrink)
                    .height(Length::Shrink),
            )
            .width(Length::Fill)
            .height(Length::Shrink)
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
                    "",
                ),
                (
                    "Date format",
                    "Show date with this format (click for more information)",
                    "https://docs.rs/chrono/latest/chrono/format/strftime/index.html",
                ),
                ("Password Generation", "You will need to restart the application", ""),
                ("Password Legth", "The number of characters in the generated password", ""),
                ("Use Special", "Allows the use of special characters, such as '*' and '&'", ""),
                ("Use Upper", "Allows the use of capital letters, such as 'A' and 'F'", ""),
                ("Use Lower", "Allows the use of lowercase letters, such as 'a' and 'j'", ""),
                ("Use Numbers", "Allows the use of numbers", ""),
            ]
            .iter()
            .flat_map(|(name, tip, url)| {
                if tip.is_empty() { return None; }
                Some(tooltip(
                    mouse_area(
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
                .into())
            })
            .collect(),
        )
        .spacing(10.)
        .width(Length::Shrink)
        .height(Length::Shrink)
    }

    pub fn list_elements(settings: &AppSettings) -> impl Into<Element<'_, MainMessage>> {
        let pwd_config = settings.password_generation();
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
            .push(
                text_input("", &settings.max_capacity().to_string()).on_input(|value| {
                    MainMessage::ChangeSettings(SettingsModified::MaxCapacity(value))
                }),
            )
            .push(
                text_input("", &settings.tick_save().to_string()).on_input(|value| {
                    MainMessage::ChangeSettings(SettingsModified::TickToSave(value))
                }),
            )
            .push(
                text_input("", &settings.shortcut())
                    .on_input(|value| {
                        MainMessage::ChangeSettings(SettingsModified::ChangeShortcut(value))
                    })
                    .style(get_input_keys_none_style()),
            )
            .push(
                text_input("", settings.format_date()).on_input(|value| {
                    MainMessage::ChangeSettings(SettingsModified::DateFormat(value))
                }),
            )
            // .push(Space::with_height(28.)) // Pasword Title
            .push(
                text("Settings")
                    .size(20.)
                    .height(28.)
                    .horizontal_alignment(iced::alignment::Horizontal::Left)
                    .vertical_alignment(iced::alignment::Vertical::Center),
            )
            .push(
                text_input("", &pwd_config.len.to_string())
                    .on_input(|value| {
                        MainMessage::ChangeSettings(SettingsModified::ChangePassLen(value))
                    })
                    .style(get_input_keys_none_style()),
            )
            .push(
                checkbox("", pwd_config.special, |value| {
                    MainMessage::ChangeSettings(SettingsModified::ChangePassUseSpecial(value))
                })
                .size(20.),
            )
            .push(
                checkbox("", pwd_config.upper, |value| {
                    MainMessage::ChangeSettings(SettingsModified::ChangePassUseUpper(value))
                })
                .size(20.),
            )
            .push(
                checkbox("", pwd_config.lower, |value| {
                    MainMessage::ChangeSettings(SettingsModified::ChangePassUseLower(value))
                })
                .size(20.),
            )
            .push(
                checkbox("", pwd_config.number, |value| {
                    MainMessage::ChangeSettings(SettingsModified::ChangePassUseNumber(value))
                })
                .size(20.),
            )
            .spacing(12.)
            .width(Length::Shrink)
            .height(Length::Shrink)
    }

    pub fn device_name(device: MDnsDevice) -> impl Into<Element<'static, MainMessage>> {
        Column::new()
            .push(
                Column::new()
                    .push(
                        text("This Device Name:")
                            .size(20.)
                            .height(28.)
                            .horizontal_alignment(iced::alignment::Horizontal::Left)
                            .vertical_alignment(iced::alignment::Vertical::Center),
                    )
                    .push(
                        text_input("", &device.name)
                            .on_input(|value| {
                                MainMessage::ChangeSettings(SettingsModified::ChangeDeviceName(
                                    value,
                                ))
                            })
                            .style(get_input_keys_none_style()),
                    )
                    .spacing(5.)
                    .width(Length::Shrink)
                    .height(Length::Shrink),
            )
            .spacing(12.)
            .width(Length::Shrink)
            .height(Length::Shrink)
    }

    pub fn devices(
        linked: Vec<MDnsDevice>,
        devices: Vec<(MDnsDevice, bool)>,
    ) -> impl Into<Element<'static, MainMessage>> {
        let mut clip = linked
            .iter()
            .enumerate()
            .rev()
            .map(|(i, d)| render_device((i, &(d.clone(), false), false)))
            .collect::<Vec<_>>();
        clip.extend(
            devices
                .iter()
                .enumerate()
                .rev()
                .map(|(i, d)| render_device((i, d, true))),
        );
        column(clip).width(Length::Fill).spacing(5)
    }
}
