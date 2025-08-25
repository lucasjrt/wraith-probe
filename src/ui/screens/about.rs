use crate::context::Context;
use crate::ui::router::RouterCommand;
use crate::{display::EspDisplay, events::AppEvent, menu::Menu};

pub struct AboutMenu<'a> {
    title: &'a str,
    content: &'a str,
}

impl AboutMenu<'_> {
    pub fn new() -> Self {
        Self {
            title: "About",
            content: "Wrait Probe.\nVersion 0.1.0\nDeveloped by JRT.",
        }
    }
}

impl Menu for AboutMenu<'_> {
    fn title(&self) -> &str {
        self.title
    }

    fn render(&self, display: &mut EspDisplay) {
        display.clear();
        display.text(&format!("=== {} ===", self.title()), 10, 10);
        display.text(self.content, 10, 30);
    }

    fn on_event(&mut self, event: &AppEvent, _ctx: &mut Context) -> Option<RouterCommand> {
        if let AppEvent::BackPressed = event {
            return Some(RouterCommand::NavigateBack);
        }
        None
    }
}
