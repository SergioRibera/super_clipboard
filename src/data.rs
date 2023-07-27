use std::fs::File;
use std::io::Read;

use abomonation::{decode, encode};
use app_dirs2::{data_root, AppDataType};

use crate::settings::{AppSettings, PinnedClipboard};

#[must_use]
pub fn load_settings() -> AppSettings {
    let Ok(mut path) = data_root(AppDataType::UserConfig) else {
        return AppSettings::default();
    };
    path.push("super_clipboard");
    path.push("settings");
    path.set_extension("data");

    log::info!("Settings Loaded!!");
    let Ok(mut file) = File::open(path) else {
        return AppSettings::default();
    };
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();

    if let Some((settings, _)) = unsafe { decode::<AppSettings>(&mut bytes) } {
        return settings.clone();
    }
    AppSettings::default()
}

pub fn save_settings(value: &AppSettings) {
    if let Ok(mut path) = data_root(AppDataType::UserConfig) {
        path.push("super_clipboard");
        path.push("settings");
        path.set_extension("data");

        log::info!("Settings saved into \"{path:?}\"");
        if let Ok(mut file) = File::create(path) {
            unsafe {
                encode(value, &mut file).unwrap();
            }
        }
    }
}

#[must_use]
pub fn load_pined() -> PinnedClipboard {
    let Ok(mut path) = data_root(AppDataType::UserConfig) else {
        return PinnedClipboard::default();
    };
    path.push("super_clipboard");
    path.push("settings");
    path.set_extension("pined");

    log::info!("Settings Loaded!!");
    let Ok(mut file) = File::open(path) else {
        return PinnedClipboard::default();
    };
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();

    if let Some((settings, _)) = unsafe { decode::<PinnedClipboard>(&mut bytes) } {
        return settings.clone();
    }
    PinnedClipboard::default()
}

pub fn save_pined(value: &PinnedClipboard) {
    if let Ok(mut path) = data_root(AppDataType::UserConfig) {
        path.push("super_clipboard");
        path.push("settings");
        path.set_extension("pined");

        log::info!("Settings saved into \"{path:?}\"");
        if let Ok(mut file) = File::create(path) {
            unsafe {
                encode(value, &mut file).unwrap();
            }
        }
    }
}
