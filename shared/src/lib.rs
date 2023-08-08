pub mod clipboard;
pub mod pinned;
pub mod mdns;

#[cfg(feature = "pc")]
pub use arboard;
pub use chrono;

#[macro_use]
pub extern crate abomonation_derive;

#[cfg(feature = "mobile")]
use mdns::*;
#[cfg(feature = "mobile")]
use clipboard::ClipboardItem;

#[cfg(feature = "mobile")]
uniffi::include_scaffolding!("mdns");

