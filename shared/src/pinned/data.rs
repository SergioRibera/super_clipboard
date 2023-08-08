use std::fs::File;
use std::io::{Read, Write};

use abomonation::{decode, encode};
use app_dirs2::{data_root, AppDataType};

use crate::pinned::PinnedClipboard;

pub fn decode_pinned(b: &[u8]) -> PinnedClipboard {
    let mut b = b.to_vec();
    if let Some((settings, _)) = unsafe { decode::<PinnedClipboard>(&mut b) } {
        return settings.clone();
    }
    PinnedClipboard::default()
}

pub fn encode_pinned(d: &PinnedClipboard) -> Vec<u8> {
    let mut b = Vec::new();
    unsafe {
        encode(d, &mut b).unwrap();
    }
    b
}

#[must_use]
pub fn load_pined() -> PinnedClipboard {
    let Ok(mut path) = data_root(AppDataType::UserConfig) else {
        return PinnedClipboard::default();
    };
    path.push("super_clipboard");
    path.push("pinned");
    path.set_extension("data");

    let Ok(mut file) = File::open(path) else {
        return PinnedClipboard::default();
    };
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();

    decode_pinned(&mut bytes)
}

pub fn save_pined(value: &PinnedClipboard) {
    if let Ok(mut path) = data_root(AppDataType::UserConfig) {
        path.push("super_clipboard");
        path.push("pinned");
        path.set_extension("data");

        if let Ok(mut file) = File::create(path) {
            file.write_all(&encode_pinned(value)).unwrap();
        }
    }
}
