use crate::{display::EspDisplay, ui::screen::Screen};

pub struct BootingScreen {
    content: Vec<String>,
}

impl BootingScreen {
    pub fn new() -> Self {
        BootingScreen { content: vec![] }
    }

    pub fn add_line(&mut self, line: &str) {
        self.content.push(line.to_string());
    }

    pub fn log(&mut self, line: &str, display: &mut EspDisplay) {
        self.add_line(line);
        self.render(display);
        display.flush();
    }
}

impl Screen for BootingScreen {
    fn title(&self) -> &str {
        "Booting..."
    }

    fn content(&self) -> &str {
        Box::leak(self.content.join("\n").into_boxed_str())
    }
}
