use crate::ui::screen::Screen;

pub struct NotImplementedScreen(String);

impl NotImplementedScreen {
    pub fn new(title: String) -> Self {
        NotImplementedScreen(title)
    }
}

impl Screen for NotImplementedScreen {
    fn title(&self) -> &str {
        &self.0
    }

    fn content(&self) -> &str {
        "This feature is not yet\nimplemented."
    }
}
