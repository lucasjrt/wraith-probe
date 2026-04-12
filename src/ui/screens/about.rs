use crate::ui::screen::Screen;

pub struct AboutMenu {
    title: &'static str,
    content: &'static str,
}

impl AboutMenu {
    pub fn new() -> Self {
        Self {
            title: "About",
            content: "Wraith Probe.\nVersion 0.1.0\nDeveloped by JRT.",
        }
    }
}

impl Screen for AboutMenu {
    fn title(&self) -> &str {
        self.title
    }

    fn content(&self) -> &str {
        self.content
    }
}
