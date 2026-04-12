use std::sync::{Arc, RwLock};

use crate::{context::Context, display::EspDisplay, ui::screen::Screen};

pub struct BootingScreen {
    content: String,
}

impl BootingScreen {
    pub fn new() -> Self {
        BootingScreen {
            content: String::new(),
        }
    }

    pub fn add_line(&mut self, line: &str) {
        if !self.content.is_empty() {
            self.content.push('\n');
        }
        self.content.push_str(line);
    }

    pub fn log(&mut self, line: &str, display: &mut EspDisplay, ctx: Arc<RwLock<Context>>) {
        self.add_line(line);
        self.render(display, ctx);
        display.flush();
    }
}

impl Screen for BootingScreen {
    fn title(&self) -> &str {
        "Booting..."
    }

    fn content(&self) -> &str {
        &self.content
    }
}
