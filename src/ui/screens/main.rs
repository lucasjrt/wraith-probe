use embedded_graphics::primitives::Rectangle;

use crate::{
    context::{Context, Dirty},
    display::EspDisplay,
    events::AppEvent,
    menu::Menu,
    ui::{
        router::RouterCommand,
        screens::{about::AboutMenu, not_implemented::NotImplementedScreen},
    },
};

pub struct MainMenu<'a> {
    title: &'a str,
    items: Vec<&'a str>,
    selected: usize,
    previous_selected: usize,
}

impl<'a> MainMenu<'a> {
    pub fn new() -> Self {
        Self {
            title: "Main Menu",
            items: vec!["Wi-Fi", "Bluetooth", "Settings", "About"],
            selected: 0,
            previous_selected: 0,
        }
    }

    pub fn items(&self) -> &Vec<&'a str> {
        &self.items
    }

    pub fn selected(&self) -> usize {
        self.selected
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
    fn title(&self) -> &str {
        self.title
    }

    fn render(&self, display: &mut EspDisplay) {
        display.text(&format!("=== {} ===", self.title()), 10, 10);
        for (i, item) in self.items().iter().enumerate() {
            if i == self.previous_selected {
                display.fill_rect(0, 30 + (i as i32 * 20) - 10, 128, 20, None);
            }

            if i == self.selected() {
                display.fill_rect(0, 30 + (i as i32 * 20) - 10, 128, 20, None);
                display.text(&format!("> {}", item), 10, 30 + (i as i32 * 20));
            } else {
                display.text(item, 10, 30 + (i as i32 * 20));
            }
        }
    }

    fn on_event(&mut self, event: &AppEvent, ctx: &mut Context) -> Option<RouterCommand> {
        match event {
            AppEvent::UpPressed => self.navigate_up(),
            AppEvent::DownPressed => self.navigate_down(),
            AppEvent::SelectPressed => match self.items[self.selected] {
                "Wi-Fi" => {
                    ctx.set_dirty(Dirty::Full);
                    return Some(RouterCommand::NavigateTo(Box::new(
                        NotImplementedScreen::new("Wi-Fi"),
                    )));
                }
                "Bluetooth" => {
                    ctx.set_dirty(Dirty::Full);
                    return Some(RouterCommand::NavigateTo(Box::new(
                        NotImplementedScreen::new("Bluetooth"),
                    )));
                }
                "Settings" => {
                    ctx.set_dirty(Dirty::Full);
                    return Some(RouterCommand::NavigateTo(Box::new(
                        NotImplementedScreen::new("Settings"),
                    )));
                }
                "About" => {
                    ctx.set_dirty(Dirty::Full);
                    return Some(RouterCommand::NavigateTo(Box::new(AboutMenu::new())));
                }
                _ => {}
            },
            _ => {
                return None;
            }
        }
        ctx.set_dirty(Dirty::Partial(Rectangle::default()));
        None
    }
}
