use clap::Parser;

use crate::settings::{load_settings, AppSettings, ThemeType};

#[derive(Clone, Parser)]
#[clap(
    author,
    version,
    about,
    long_about = "This cli args are overwrite the current configuration"
)]
pub struct AppArgs {
    #[clap(long, short, help = "Max capacity of history")]
    pub(self) max_capacity: Option<u64>,
    #[clap(
        long,
        help = "Define interval verification to save changes on settings (in milliseconds)"
    )]
    pub(self) tick_save: Option<u64>,
    #[clap(long, help = "Set GUI theme")]
    pub(self) theme: Option<ThemeType>,
    #[clap(long, short, help = "Set date custom formated")]
    pub(self) format_date: Option<String>,
    #[clap(long, short, help = "Allow save history clipboard session")]
    pub(self) store: Option<bool>,
    #[clap(
        long,
        short,
        help = "Allow preserve the externally removed clipboard. Ideally used by password managers"
    )]
    pub(self) preserve: Option<bool>,
    #[clap(long, short, help = "Set application transparent")]
    pub(self) transparent: Option<bool>,
}

pub fn parse_cli() -> AppSettings {
    let args = AppArgs::parse();
    log::trace!("CLI Arguments parsed");
    let mut settings = load_settings();
    log::info!("Loaded saved settings");

    if let Some(v) = args.max_capacity {
        settings.set_max_capacity(v);
    }

    if let Some(v) = args.tick_save {
        settings.set_tick_save(v);
    }

    if let Some(v) = args.theme {
        settings.set_theme(v);
    }

    if let Some(v) = args.format_date {
        settings.set_format_date(v);
    }

    if let Some(v) = args.store {
        if v != settings.store() {
            settings.set_store(v);
        }
    }

    if let Some(v) = args.preserve {
        if v == settings.preserve() {
            settings.set_preserve(v);
        }
    }

    if let Some(v) = args.transparent {
        if v != settings.transparent() {
            settings.set_transparent(v);
        }
    }

    settings
}
