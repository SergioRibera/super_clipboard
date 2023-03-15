#![allow(unused)]
use arboard::ImageData;
use chrono::prelude::*;
use preferences::Preferences;
use serde::{Deserialize, Serialize};

use crate::APPINFO;

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct AppSettings {
    max_capacity: u64,
    tick_save: u64,
    theme: ThemeType,
    store: bool,
    transparent: bool,
    format_date: String,
    clipboard: Vec<ClipboardItem>,
    #[serde(skip)]
    pub is_changed: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ThemeType {
    Light,
    Dark,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ClipboardItem {
    Text(DateTime<Utc>, String),
    Image(DateTime<Utc>, usize, usize, Vec<u8>),
}

#[must_use]
pub fn load_settings() -> AppSettings {
    AppSettings::load(&APPINFO, "settings").unwrap_or(AppSettings::default())
}

pub fn save_settings(value: &AppSettings) {
    value.save(&APPINFO, "settings").unwrap()
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            max_capacity: 10000,
            tick_save: 2000,
            theme: ThemeType::Dark,
            store: true,
            transparent: true,
            is_changed: false,
            format_date: "%d %b %Y - %H:%M:%S".to_string(),
            clipboard: Vec::new(),
        }
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
    pub fn set_transparent(&mut self, transparent: bool) {
        self.transparent = transparent;
        self.is_changed = true;
    }

    pub fn set_format_date(&mut self, format_date: String) {
        self.format_date = format_date;
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

    pub fn clipboard(&self) -> &[ClipboardItem] {
        self.clipboard.as_ref()
    }

    pub fn push(&mut self, item: ClipboardItem) {
        self.clipboard.push(item);
        if self.store {
            self.is_changed = true;
        }
    }

    pub fn remove(&mut self, i: usize) {
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
}

impl ThemeType {
    pub fn toggle_str(&self) -> String {
        match self {
            ThemeType::Light => "Dark".to_string(),
            ThemeType::Dark => "Light".to_string(),
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
        Self::Text(Utc::now(), value)
    }
}

impl From<ImageData<'_>> for ClipboardItem {
    fn from(value: ImageData) -> Self {
        let ImageData {
            width,
            height,
            bytes,
        } = value;
        Self::Image(Utc::now(), width, height, bytes.to_vec())
    }
}
