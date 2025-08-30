use crate::{display::EspDisplay, ui::screen::Screen};

pub struct NotImplementedScreen(&'static str);

impl NotImplementedScreen {
    pub fn new(title: &'static str) -> Self {
        NotImplementedScreen(title)
    }
}

impl Screen for NotImplementedScreen {
    fn title(&self) -> &str {
        self.0
    }

    fn content(&self) -> &str {
        "This feature is not yet\nimplemented."
    }
}
