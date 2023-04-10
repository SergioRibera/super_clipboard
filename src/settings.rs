#![allow(unused)]
use std::path::PathBuf;

use arboard::ImageData;
use chrono::prelude::*;
use clap::ValueEnum;
use log::info;
use preferences::Preferences;
use serde::{Deserialize, Serialize};

use crate::APPINFO;

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct AppSettings {
    max_capacity: u64,
    tick_save: u64,
    theme: ThemeType,
    store: bool,
    preserve: bool,
    transparent: bool,
    format_date: String,
    activation_keys: Vec<String>,
    clipboard: Vec<ClipboardItem>,
    #[serde(skip)]
    pub is_changed: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ValueEnum)]
pub enum ThemeType {
    Light,
    Dark,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ClipboardItem {
    Text(DateTime<Utc>, String),
    Image(DateTime<Utc>, usize, usize, uuid::Uuid),
}

#[must_use]
pub fn load_settings() -> AppSettings {
    AppSettings::load(&APPINFO, "settings").unwrap_or_default()
}

pub fn save_settings(value: &AppSettings) {
    value.save(&APPINFO, "settings").unwrap()
}

pub fn image_path(id: uuid::Uuid) -> String {
    let binding = preferences::prefs_base_dir()
        .unwrap()
        .join(APPINFO.name)
        .join("images");
    if !binding.exists() {
        std::fs::create_dir_all(&binding);
    }
    let binding = binding.join(&format!("{id:?}.png"));
    info!("Saving image on \"{binding:?}\" path");
    let path = binding.to_str().unwrap();
    path.to_string()
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
            format_date: "%d %b %Y - %H:%M:%S".to_string(),
            activation_keys: vec!["LShift".to_string(), "V".to_string(), "Meta".to_string()],
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

    pub fn set_shortcut(&mut self, keys: Vec<String>) {
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

    pub fn shortcut(&self) -> &[String] {
        self.activation_keys.as_ref()
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
        match self.clipboard.remove(i) {
            ClipboardItem::Image(_, _, _, id) => std::fs::remove_file(image_path(id)).unwrap(),
            _ => {}
        }
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

impl TryFrom<ImageData<'_>> for ClipboardItem {
    type Error = String;

    fn try_from(value: ImageData) -> Result<Self, Self::Error> {
        let ImageData {
            width,
            height,
            bytes,
        } = value;
        let id = uuid::Uuid::from_slice(&bytes[0..16]).unwrap();
        let img_path = image_path(id.clone());
        match image::RgbImage::from_raw(width as u32, height as u32, bytes.to_vec()) {
            Some(img) => {
                img.save(img_path).unwrap();
                return Ok(Self::Image(Utc::now(), width, height, id));
            }
            None => Err(format!("Cannot load image from_raw")),
        }
    }
}
