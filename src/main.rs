#![windows_subsystem = "windows"]

mod args;
mod daemon;
mod gui;
mod settings;
mod ui;
mod update;

use args::AppArgs;
use clap::Parser;
use iced::{Application, Settings};
use preferences::AppInfo;
use settings::load_settings;
use ui::MainApp;

pub const APPINFO: AppInfo = AppInfo {
    name: env!("CARGO_PKG_NAME"),
    author: env!("CARGO_PKG_AUTHORS"),
};

pub const APP_WIDTH: i32 = 350;
pub const APP_HEIGHT: i32 = 450;
pub const APP_MOUSE_MARGIN: i32 = 25;

fn main() -> iced::Result {
    let _args = AppArgs::parse();
    let settings = load_settings();

    MainApp::run(Settings {
        window: iced::window::Settings {
            size: (APP_WIDTH as u32, APP_HEIGHT as u32),
            visible: true,
            resizable: false,
            decorations: false,
            transparent: settings.transparent(),
            always_on_top: true,
            ..Default::default()
        },
        flags: settings,
        ..Default::default()
    })
}
