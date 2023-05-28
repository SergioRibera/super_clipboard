#![windows_subsystem = "windows"]

#[macro_use]
extern crate abomonation_derive;

mod args;
mod daemon;
mod gui;
mod keys;
mod settings;
mod ui;
mod update;

use args::parse_cli;
use iced::{Application, Settings};
use ui::MainApp;

#[global_allocator]
static A: rlsf::SmallGlobalTlsf = rlsf::SmallGlobalTlsf::new();

pub const APP_WIDTH: i32 = 350;
pub const APP_HEIGHT: i32 = 450;
pub const APP_MOUSE_MARGIN: i32 = 25;

fn main() -> iced::Result {
    env_logger::Builder::from_env(env_logger::Env::new().filter_or("SUPER_CLIPBOARD_LOG", "warn"))
        .init();
    let settings = parse_cli();

    log::trace!("Before to run Application");

    MainApp::run(Settings {
        window: iced::window::Settings {
            size: (APP_WIDTH as u32, APP_HEIGHT as u32),
            visible: false,
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
