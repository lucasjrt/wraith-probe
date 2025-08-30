use embedded_graphics::primitives::Rectangle;

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

pub struct MainMenu<'a> {
    title: &'a str,
    items: Vec<&'a str>,
    selected: usize,
    previous_selected: usize,
}

impl MainMenu<'_> {
    pub fn new() -> Self {
        Self {
            title: "Main Menu",
            items: vec![
                "Wi-Fi",
                "Bluetooth",
                "Settings",
                "About",
                "Exit But Actually not the real, only for horizontal menu testing",
                "Restart",
                "Shutdown",
                "Sleep",
                "Factory Reset",
            ],
            selected: 0,
            previous_selected: 0,
        }
    }

    pub fn navigate_up(&mut self) {
        self.previous_selected = self.selected;
        self.selected = (self.selected + self.items.len() - 1) % self.items.len();
    }

    pub fn navigate_down(&mut self) {
        self.previous_selected = self.selected;
        self.selected = (self.selected + 1) % self.items.len();
    }
}

impl Menu for MainMenu<'_> {
    fn items(&self) -> &[&str] {
        &self.items
    }

    fn selected(&self) -> usize {
        self.selected
    }

    fn previous_selected(&self) -> usize {
        self.previous_selected
    }
}

impl Screen for MainMenu<'_> {
    fn on_event(&mut self, event: &AppEvent, ctx: &mut Context) -> Option<RouterCommand> {
        match event {
            AppEvent::UpPressed => self.navigate_up(),
            AppEvent::DownPressed => self.navigate_down(),
            AppEvent::SelectPressed => match self.items[self.selected] {
                "About" => {
                    ctx.set_dirty(Dirty::Full);
                    return Some(RouterCommand::NavigateTo(Box::new(AboutMenu::new())));
                }
                text => {
                    ctx.set_dirty(Dirty::Full);
                    return Some(RouterCommand::NavigateTo(Box::new(
                        NotImplementedScreen::new(String::from(text)),
                    )));
                }
            },
            _ => {
                return None;
            }
        }
        ctx.set_dirty(Dirty::Partial(Rectangle::default()));
        None
    }

    fn title(&self) -> &str {
        self.title
    }

    fn menu(&self) -> Option<&dyn Menu> {
        Some(self)
    }
}
