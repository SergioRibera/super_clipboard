#![allow(unused)]

use app_dirs2::{data_root, AppDataType};
use arboard::ImageData;
use chrono::prelude::*;
use clap::ValueEnum;

use crate::data::save_pined;

#[derive(Abomonation, Clone, Eq, PartialEq)]
pub struct AppSettings {
    max_capacity: u64,
    tick_save: u64,
    theme: ThemeType,
    store: bool,
    preserve: bool,
    transparent: bool,
    format_date: String,
    activation_keys: String,
    clipboard: Vec<ClipboardItem>,
    password_generation: PasswordGenSettings,
    #[unsafe_abomonate_ignore]
    pub is_changed: bool,
}

#[derive(Abomonation, Clone, Default, Eq, PartialEq)]
pub struct PinnedClipboard {
    clipboard: Vec<ClipboardItem>,
    #[unsafe_abomonate_ignore]
    pub is_changed: bool,
}

#[derive(Abomonation, Clone, Eq, PartialEq)]
pub struct PasswordGenSettings {
    pub len: usize,
    pub special: bool,
    pub upper: bool,
    pub lower: bool,
    pub number: bool,
}

#[derive(Abomonation, Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum ThemeType {
    Light,
    Dark,
}

#[derive(Abomonation, Clone, Debug, Eq, PartialEq)]
pub enum ClipboardItem {
    Text(String, String),
    Image(String, usize, usize, Vec<u8>),
}

impl Default for AppSettings {
    fn default() -> Self {
        let mode = dark_light::detect();
        let theme = match mode {
            // Dark mode
            dark_light::Mode::Dark => ThemeType::Dark,
            // Light mode
            dark_light::Mode::Light => ThemeType::Light,
            // Unspecified
            dark_light::Mode::Default => ThemeType::Dark,
        };

        Self {
            theme,
            max_capacity: 10000,
            tick_save: 2000,
            transparent: true,
            store: true,
            preserve: false,
            is_changed: false,
            password_generation: PasswordGenSettings::default(),
            format_date: "%d %b %Y - %H:%M:%S".to_string(),
            activation_keys: "super+shift+v".to_string(),
            clipboard: Vec::new(),
        }
    }
}

impl PinnedClipboard {
    pub fn clipboard(&self) -> &[ClipboardItem] {
        self.clipboard.as_ref()
    }

    pub fn add_item(&mut self, item: ClipboardItem) {
        self.clipboard.push(item);
        self.is_changed = true;
    }

    pub fn remove_item(&mut self, pos: usize) {
        self.clipboard.remove(pos);
        self.is_changed = true;
    }
}

impl AppSettings {
    pub fn set_max_capacity(&mut self, max_capacity: u64) {
        self.max_capacity = max_capacity;
        self.is_changed = true;
    }

    pub fn set_tick_save(&mut self, tick_save: u64) {
        self.tick_save = tick_save;
        self.is_changed = true;
    }

    pub fn set_theme(&mut self, theme: ThemeType) {
        self.theme = theme;
        self.is_changed = true;
    }

    pub fn set_store(&mut self, store: bool) {
        self.store = store;
        self.is_changed = true;
    }

    pub fn set_preserve(&mut self, preserve: bool) {
        self.preserve = preserve;
        self.is_changed = true;
    }

    pub fn set_transparent(&mut self, transparent: bool) {
        self.transparent = transparent;
        self.is_changed = true;
    }

    pub fn set_format_date(&mut self, format_date: String) {
        self.format_date = format_date;
        self.is_changed = true;
    }

    pub fn set_shortcut(&mut self, keys: String) {
        self.activation_keys = keys;
        self.is_changed = true;
    }

    pub fn set_clipboard(&mut self, clipboard: Vec<ClipboardItem>) {
        self.clipboard = clipboard;
        if self.store {
            self.is_changed = true;
        }
    }

    pub fn toggle_theme(&mut self) {
        self.theme = match self.theme {
            ThemeType::Light => ThemeType::Dark,
            ThemeType::Dark => ThemeType::Light,
        };
        self.is_changed = true;
    }

    pub fn max_capacity(&self) -> u64 {
        self.max_capacity
    }

    pub fn tick_save(&self) -> u64 {
        self.tick_save
    }

    pub fn get_theme(&self) -> &ThemeType {
        &self.theme
    }

    pub fn format_date(&self) -> &str {
        self.format_date.as_ref()
    }

    pub fn transparent(&self) -> bool {
        self.transparent
    }

    pub fn store(&self) -> bool {
        self.store
    }

    pub fn preserve(&self) -> bool {
        self.preserve
    }

    pub fn shortcut(&self) -> String {
        self.activation_keys.clone()
    }

    pub fn clipboard(&self) -> &[ClipboardItem] {
        self.clipboard.as_ref()
    }

    pub fn push(&mut self, item: ClipboardItem) {
        // check for repeated entries and delete the older entry to prevent duplicates.
        let repeated_index = self
            .clipboard
            .iter()
            .position(|previous_item| item.to_string() == previous_item.to_string());
        if let Some(repeated_index) = repeated_index {
            self.clipboard.remove(repeated_index);
        }
        if self.clipboard.len() + 1 >= self.max_capacity as usize {
            self.clipboard.remove(0);
        }
        self.clipboard.push(item);
        if self.store {
            self.is_changed = true;
        }
    }

    pub fn remove(&mut self, i: usize) {
        if self.clipboard.is_empty() || i >= self.clipboard.len() {
            return;
        }
        self.clipboard.remove(i);
        if self.store {
            self.is_changed = true;
        }
    }

    pub fn clear(&mut self) {
        self.clipboard.clear();
        if self.store {
            self.is_changed = true;
        }
    }

    pub fn password_generation(&self) -> &PasswordGenSettings {
        &self.password_generation
    }

    pub fn password_generation_mut(&mut self) -> &mut PasswordGenSettings {
        if self.store {
            self.is_changed = true;
        }
        &mut self.password_generation
    }

    pub fn set_password_generation(&mut self, password_generation: PasswordGenSettings) {
        self.password_generation = password_generation;
    }
}

impl ThemeType {
    pub fn toggle_str(&self) -> String {
        match self {
            ThemeType::Light => "Dark".to_string(),
            ThemeType::Dark => "Light".to_string(),
        }
    }
}

impl Default for PasswordGenSettings {
    fn default() -> Self {
        Self {
            len: 12,
            special: true,
            upper: true,
            lower: true,
            number: true,
        }
    }
}

impl ClipboardItem {
    pub fn format(&self, fmt: &str) -> String {
        match self {
            ClipboardItem::Text(date, _) => DateTime::parse_from_rfc3339(date)
                .unwrap()
                .format(fmt)
                .to_string(),
            // @TODO:Convert to base64
            ClipboardItem::Image(date, _, _, _) => DateTime::parse_from_rfc3339(date)
                .unwrap()
                .format(fmt)
                .to_string(),
        }
    }
}

impl ToString for ClipboardItem {
    fn to_string(&self) -> String {
        match self {
            ClipboardItem::Text(_date, v) => v.clone(),
            // @TODO:Convert to base64
            ClipboardItem::Image(_date, w, h, _b) => format!("{w} - {h}"),
        }
    }
}

impl From<String> for ClipboardItem {
    fn from(value: String) -> Self {
        Self::Text(Utc::now().to_rfc3339(), value)
    }
}

impl From<ImageData<'_>> for ClipboardItem {
    fn from(value: ImageData) -> Self {
        let ImageData {
            width,
            height,
            bytes,
        } = value;
        Self::Image(Utc::now().to_rfc3339(), width, height, bytes.to_vec())
    }
}
