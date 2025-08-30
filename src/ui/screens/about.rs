use crate::ui::screen::Screen;

pub struct AboutMenu<'a> {
    title: &'a str,
    content: &'a str,
}

impl AboutMenu<'_> {
    pub fn new() -> Self {
        Self {
            title: "About",
            content: "Wraith Probe.\nVersion 0.1.0\nDeveloped by JRT.",
        }
    }
}

impl Screen for AboutMenu<'_> {
    fn title(&self) -> &str {
        self.title
    }

    fn content(&self) -> &str {
        self.content
    }
}
