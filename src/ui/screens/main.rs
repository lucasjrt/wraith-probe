use std::sync::{Arc, RwLock};

use embedded_graphics::primitives::Rectangle;
use esp_idf_svc::timer::EspTimer;

use crate::{
    context::{Context, Dirty},
    events::AppEvent,
    menu::Menu,
    ui::{
        router::RouterCommand,
        screen::Screen,
        screens::{about::AboutMenu, not_implemented::NotImplementedScreen},
    },
};

pub struct MainMenu {
    title: &'static str,
    items: Vec<&'static str>,
    selected: usize,
    previous_selected: usize,
    scroll_offset: usize,
    horizontal_scroll_offset: i32,
    scroll_timer: Option<EspTimer<'static>>,
}

impl MainMenu {
    pub fn new() -> Self {
        Self {
            title: "Main Menu",
            items: vec![
                "Wi-Fi",
                "Bluetooth",
                "Settings",
                "About",
                "Exit But Actually not the real, only for horizontal menu testing",
                "Another Long Item For Testing Scroll Reset Behavior",
                "Restart",
                "Shutdown",
                "Sleep",
                "Factory Reset",
            ],
            selected: 0,
            previous_selected: 0,
            scroll_offset: 0,
            horizontal_scroll_offset: 0,
            scroll_timer: None,
        }
    }

    pub fn navigate_up(&mut self) {
        self.previous_selected = self.selected;
        self.horizontal_scroll_offset = 0;
        self.scroll_timer = None;
        self.selected = (self.selected + self.items.len() - 1) % self.items.len();
    }

    pub fn navigate_down(&mut self) {
        self.previous_selected = self.selected;
        self.horizontal_scroll_offset = 0;
        self.scroll_timer = None;
        self.selected = (self.selected + 1) % self.items.len();
    }
}

impl Menu for MainMenu {
    fn items(&self) -> &[&str] {
        &self.items
    }

    fn selected(&self) -> usize {
        self.selected
    }

    fn previous_selected(&self) -> usize {
        self.previous_selected
    }

    fn menu_scroll_offset(&self) -> usize {
        self.scroll_offset
    }

    fn set_menu_scroll_offset(&mut self, offset: usize) {
        self.scroll_offset = offset;
    }

    fn selected_scroll_offset(&self) -> i32 {
        self.horizontal_scroll_offset
    }

    fn set_selected_scroll_offset(&mut self, offset: i32) {
        self.horizontal_scroll_offset = offset;
    }

    fn scroll_timer(&self) -> &Option<EspTimer<'static>> {
        &self.scroll_timer
    }

    fn set_scroll_timer(&mut self, timer: Option<EspTimer<'static>>) {
        self.scroll_timer = timer;
    }
}

impl Screen for MainMenu {
    fn on_event(&mut self, event: &AppEvent, ctx: Arc<RwLock<Context>>) -> Option<RouterCommand> {
        match event {
            AppEvent::UpPressed => self.navigate_up(),
            AppEvent::DownPressed => self.navigate_down(),
            AppEvent::SelectPressed => match self.items[self.selected] {
                "About" => {
                    if let Ok(mut ctx) = ctx.write() {
                        ctx.set_dirty(Dirty::Full);
                    }
                    return Some(RouterCommand::NavigateTo(Box::new(AboutMenu::new())));
                }
                text => {
                    if let Ok(mut ctx) = ctx.write() {
                        ctx.set_dirty(Dirty::Full);
                    }
                    return Some(RouterCommand::NavigateTo(Box::new(
                        NotImplementedScreen::new(String::from(text)),
                    )));
                }
            },
            _ => {
                return None;
            }
        }
        {
            if let Ok(mut ctx) = ctx.write() {
                ctx.set_dirty(Dirty::Partial(Rectangle::default()));
            }
        }
        None
    }

    fn title(&self) -> &str {
        self.title
    }

    fn menu(&self) -> Option<&dyn Menu> {
        Some(self)
    }

    fn menu_mut(&mut self) -> Option<&mut dyn Menu> {
        Some(self)
    }
}
