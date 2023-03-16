use std::time::{Duration, Instant};

use arboard::Clipboard;
use iced::widget::scrollable::Properties;
use iced::widget::{svg, Row};
use iced::{
    widget::{container, scrollable, Column},
    Element, Length, Theme,
};
use iced::{Application, Color, Command, Padding, Subscription};

use crate::daemon;
use crate::gui::{home, settings};
use crate::settings::ThemeType;
use crate::settings::{AppSettings, ClipboardItem};
use crate::update::handle_update;

use self::mouse_listener::mouse_listener;

pub mod item;
pub mod mouse_listener;
pub mod styles;

pub struct MainApp {
    pub settings: AppSettings,
    pub clipboard_ctx: Clipboard,
    pub view: RouterView,
    pub visible: bool,
    pub follow: bool,
    tip_icon: svg::Handle,
    dark_icon: svg::Handle,
    light_icon: svg::Handle,
    trash_icon: svg::Handle,
    settings_icon: svg::Handle,
}

#[derive(Debug, Clone)]
pub enum RouterView {
    Home,
    Settings,
}

#[derive(Debug, Clone)]
pub enum MainMessage {
    ClearClipboard,
    ThemeChangedToggle,
    Open(String),
    ChangeSettings(SettingsModified),
    ChangeView(RouterView),
    CheckSettings(Instant),
    DaemonEvent(daemon::Event),
    RemoveClipboard(usize),
    SetClipboard(ClipboardItem),
}

#[derive(Debug, Clone)]
pub enum SettingsModified {
    MaxCapacity(String),
    TickToSave(String),
    StoreClipboard(bool),
    ChangeTransparency(bool),
    DateFormat(String),
    ChangeShortcut(String),
}

impl Application for MainApp {
    type Message = MainMessage;
    type Executor = iced::executor::Default;
    type Theme = Theme;
    type Flags = AppSettings;

    fn new(settings: Self::Flags) -> (Self, Command<Self::Message>) {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        (
            Self {
                settings,
                visible: true,
                follow: false,
                view: RouterView::Home,
                clipboard_ctx: Clipboard::new().unwrap(),
                tip_icon: svg::Handle::from_path(format!("{manifest_dir}/assets/hint.svg")),
                dark_icon: svg::Handle::from_path(format!("{manifest_dir}/assets/night-mode.svg")),
                light_icon: svg::Handle::from_path(format!("{manifest_dir}/assets/light-mode.svg")),
                trash_icon: svg::Handle::from_path(format!("{manifest_dir}/assets/trash.svg")),
                settings_icon: svg::Handle::from_path(format!(
                    "{manifest_dir}/assets/settings.svg"
                )),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Super Clipboard")
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        Subscription::batch(vec![
            daemon::start_daemon(self.settings.shortcut()).map(MainMessage::DaemonEvent),
            iced::time::every(Duration::from_millis(self.settings.tick_save()))
                .map(MainMessage::CheckSettings),
        ])
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        handle_update(self, message)
    }

    fn view(&self) -> Element<MainMessage> {
        let content: Element<MainMessage> = match self.view {
            RouterView::Home => Column::new()
                .push(home::top_bar(
                    self.settings.get_theme(),
                    self.dark_icon.clone(),
                    self.light_icon.clone(),
                    self.trash_icon.clone(),
                    self.settings_icon.clone(),
                ))
                .push(home::tip_section(self.tip_icon.clone()))
                .push(
                    scrollable(home::show_items(
                        self.settings.format_date(),
                        self.settings.clipboard(),
                    ))
                    .vertical_scroll(Properties::default().width(5.).scroller_width(5.))
                    .height(Length::Fill),
                )
                .spacing(10)
                .padding(10)
                .width(Length::Fill)
                .height(Length::Fill)
                .into(),
            RouterView::Settings => Column::new()
                .push(settings::back_bar(self.tip_icon.clone()))
                .push(settings::tip_section(self.tip_icon.clone()))
                .push(
                    Row::new()
                        .push(settings::list_options())
                        .push(settings::list_elements(&self.settings))
                        .spacing(10)
                        .padding(10),
                )
                .padding(Padding::from([10, 0]))
                .width(Length::Fill)
                .height(Length::Fill)
                .into(),
        };

        container(
            mouse_listener(content)
                .on_mouse_exit(MainMessage::DaemonEvent(daemon::Event::Message(
                    daemon::Message::ToggleVisibility(false),
                ))),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }

    fn theme(&self) -> Theme {
        let primary = Color::from_rgba8(0x77, 0x8f, 0x9b, 1.);
        match self.settings.get_theme() {
            ThemeType::Light => {
                let palette = Theme::Light.palette();
                Theme::custom(iced::theme::Palette {
                    primary,
                    background: Color::from_rgba8(200, 200, 200, 0.025),
                    ..palette
                })
            }
            ThemeType::Dark => {
                let palette = Theme::Dark.palette();
                let bg = palette.background;
                Theme::custom(iced::theme::Palette {
                    primary,
                    background: Color::from_rgba(bg.r, bg.g, bg.b, 0.3),
                    ..palette
                })
            }
        }
    }
}
