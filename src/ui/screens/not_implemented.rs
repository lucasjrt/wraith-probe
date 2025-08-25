use crate::{
    context::Context, display::EspDisplay, events::AppEvent, menu::Menu, ui::router::RouterCommand,
};

pub struct NotImplementedScreen;

impl NotImplementedScreen {
    pub fn new() -> Self {
        NotImplementedScreen
    }
}

impl Menu for NotImplementedScreen {
    fn title(&self) -> &str {
        "Not Implemented"
    }

    fn render(&self, display: &mut EspDisplay) {
        display.clear();
        display.text("=== Not Implemented ===", 10, 10);
        display.text("This feature is not yet\nimplemented.", 10, 30);
    }

    fn on_event(&mut self, event: &AppEvent, _ctx: &mut Context) -> Option<RouterCommand> {
        if let AppEvent::BackPressed = event {
            return Some(RouterCommand::NavigateBack);
        }
        None
    }
}
