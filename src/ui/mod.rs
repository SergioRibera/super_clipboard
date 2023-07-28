use std::str::FromStr;
use std::time::{Duration, Instant};

use arboard::Clipboard;
use device_query::DeviceState;
use global_hotkey::hotkey::HotKey;
use global_hotkey::GlobalHotKeyManager;
use iced::widget::scrollable::Properties;
use iced::widget::{svg, Row};
use iced::{
    widget::{container, scrollable, Column},
    Element, Length, Theme,
};
use iced::{Application, Color, Command, Padding, Subscription};
use iced_native::subscription::events_with;
use log::{info, trace};

use crate::data::load_pined;
use crate::gui::{home, settings};
use crate::passwd::PasswordGenerator;
use crate::settings::{AppSettings, ClipboardItem};
use crate::settings::{PinnedClipboard, ThemeType};
use crate::update::handle_update;

pub mod item;
pub mod styles;

pub struct MainApp {
    pub settings: AppSettings,
    pub pinned: PinnedClipboard,
    pub clipboard_ctx: Clipboard,
    pub device_state: DeviceState,
    pub view: RouterView,
    pub visible: bool,
    pub follow: bool,
    pub password_generator: PasswordGenerator,
    pub hotkeys_manager: GlobalHotKeyManager,
    pub hotkey: HotKey,
    pub last_data: LastData,
    back_icon: svg::Handle,
    tip_icon: svg::Handle,
    dark_icon: svg::Handle,
    light_icon: svg::Handle,
    gen_password_icon: svg::Handle,
    trash_icon: svg::Handle,
    settings_icon: svg::Handle,
    pin_icon: svg::Handle,
    unpin_icon: svg::Handle,
}

pub struct LastData {
    pub last_str: String,
    pub last_image: (usize, usize, usize, [u8; 2]),
    pub mouse_pos: (i32, i32),
}

#[derive(Debug, Clone)]
pub enum RouterView {
    Home,
    Settings,
}

#[derive(Debug, Clone)]
pub enum MainMessage {
    GeneratePassword,
    ClearClipboard,
    ThemeChangedToggle,
    HiddeApplication,
    Open(String),
    ChangeSettings(SettingsModified),
    ChangeView(RouterView),
    CheckClipboard(Instant),
    CheckSettings(Instant),
    CheckShortcuts(Instant),
    RemoveClipboard(usize),
    SetClipboard(ClipboardItem),
    // Pin
    TogglePinClipboard(Option<usize>, Option<ClipboardItem>),
}

#[derive(Debug, Clone)]
pub enum SettingsModified {
    MaxCapacity(String),
    TickToSave(String),
    StoreClipboard(bool),
    StorePreserve(bool),
    ChangeTransparency(bool),
    DateFormat(String),
    ChangeShortcut(String),
    // Password Genration
    ChangePassLen(String),
    ChangePassUseSpecial(bool),
    ChangePassUseUpper(bool),
    ChangePassUseLower(bool),
    ChangePassUseNumber(bool),
}

impl Application for MainApp {
    type Message = MainMessage;
    type Executor = iced::executor::Default;
    type Theme = Theme;
    type Flags = AppSettings;

    fn new(settings: Self::Flags) -> (Self, Command<Self::Message>) {
        trace!("Creating Iced Application");

        let password_generator = PasswordGenerator::new(settings.password_generation());
        let hotkeys_manager = GlobalHotKeyManager::new().unwrap();
        let hotkey = HotKey::from_str(&settings.shortcut()).unwrap();
        hotkeys_manager.register(hotkey).unwrap();

        log::info!("Hotkey registered: {hotkey:?}");

        let mut clipboard_ctx = Clipboard::new().unwrap();
        let last_str = clipboard_ctx.get_text().unwrap_or_default();

        (
            Self {
                hotkey,
                settings,
                clipboard_ctx,
                hotkeys_manager,
                password_generator,
                visible: true,
                follow: false,
                view: RouterView::Home,
                device_state: DeviceState::new(),
                pinned: load_pined(),
                last_data: LastData {
                    last_str,
                    last_image: (0usize, 0usize, 0usize, [0u8; 2]),
                    mouse_pos: (0, 0),
                },
                back_icon: svg::Handle::from_memory(
                    include_bytes!("../../assets/back.svg").to_vec(),
                ),
                tip_icon: svg::Handle::from_memory(
                    include_bytes!("../../assets/hint.svg").to_vec(),
                ),
                dark_icon: svg::Handle::from_memory(
                    include_bytes!("../../assets/night-mode.svg").to_vec(),
                ),
                light_icon: svg::Handle::from_memory(
                    include_bytes!("../../assets/light-mode.svg").to_vec(),
                ),
                gen_password_icon: svg::Handle::from_memory(
                    include_bytes!("../../assets/password.svg").to_vec(),
                ),
                trash_icon: svg::Handle::from_memory(
                    include_bytes!("../../assets/trash.svg").to_vec(),
                ),
                settings_icon: svg::Handle::from_memory(
                    include_bytes!("../../assets/settings.svg").to_vec(),
                ),
                pin_icon: svg::Handle::from_memory(
                    include_bytes!("../../assets/pin.svg").to_vec(),
                ),
                unpin_icon: svg::Handle::from_memory(
                    include_bytes!("../../assets/unpin.svg").to_vec(),
                ),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Super Clipboard")
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        trace!("Subscription Batch");
        Subscription::batch(vec![
            // Check Shortcuts
            iced::time::every(Duration::from_millis(200)).map(MainMessage::CheckShortcuts),
            // Check Clipboard
            iced::time::every(Duration::from_secs(1)).map(MainMessage::CheckClipboard),
            // Check Changed Settings
            iced::time::every(Duration::from_millis(self.settings.tick_save()))
                .map(MainMessage::CheckSettings),
            events_with(|e, _status| match e {
                iced_native::Event::Mouse(iced_native::mouse::Event::CursorLeft) => {
                    Some(MainMessage::HiddeApplication)
                }
                _ => None,
            }),
        ])
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        info!("Update Commands");
        handle_update(self, message)
    }

    fn view(&self) -> Element<MainMessage> {
        trace!("Render Application");
        let content: Element<MainMessage> = match self.view {
            RouterView::Home => Column::new()
                .push(home::top_bar(
                    self.settings.get_theme(),
                    self.dark_icon.clone(),
                    self.light_icon.clone(),
                    self.gen_password_icon.clone(),
                    self.trash_icon.clone(),
                    self.settings_icon.clone(),
                ))
                .push(home::tip_section(self.tip_icon.clone()))
                .push(
                    scrollable(home::show_items(
                        self.settings.format_date(),
                        self.settings.clipboard(),
                        self.pinned.clipboard(),
                        self.pin_icon.clone(),
                        self.unpin_icon.clone(),
                    ))
                    .height(Length::Fill)
                    .vertical_scroll(Properties::new().width(5.).scroller_width(5.)),
                )
                .spacing(10)
                .padding(10)
                .width(Length::Fill)
                .height(Length::Fill)
                .into(),
            RouterView::Settings => Column::new()
                .push(settings::back_bar(self.back_icon.clone()))
                .push(settings::tip_section(self.tip_icon.clone()))
                .push(
                    scrollable(
                        Row::new()
                            .push(settings::list_options())
                            .push(settings::list_elements(&self.settings))
                            .spacing(10)
                            .padding(10),
                    )
                    .height(Length::Fill)
                    .vertical_scroll(Properties::new().width(5.).scroller_width(5.)),
                )
                .padding(Padding::from([10, 0]))
                .width(Length::Fill)
                .height(Length::Fill)
                .into(),
        };

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Theme {
        trace!("Set Theme");
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
