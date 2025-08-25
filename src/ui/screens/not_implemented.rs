use crate::{
    context::Context, display::EspDisplay, events::AppEvent, menu::Menu, ui::router::RouterCommand,
};

pub struct NotImplementedScreen(&'static str);

impl NotImplementedScreen {
    pub fn new(title: &'static str) -> Self {
        NotImplementedScreen(title)
    }
}

impl Menu for NotImplementedScreen {
    fn title(&self) -> &str {
        self.0
    }

    fn render(&self, display: &mut EspDisplay) {
        display.clear();
        display.text(&format!("=== {} ===", self.title()), 10, 10);
        display.text("This feature is not yet\nimplemented.", 10, 30);
    }

    fn on_event(&mut self, event: &AppEvent, _ctx: &mut Context) -> Option<RouterCommand> {
        if let AppEvent::BackPressed = event {
            return Some(RouterCommand::NavigateBack);
        }
        None
    }
}
