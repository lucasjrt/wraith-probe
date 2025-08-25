use crate::{context::Context, display::EspDisplay, events::AppEvent, ui::router::RouterCommand};

pub trait Menu {
    fn title(&self) -> &str;
    fn render(&self, display: &mut EspDisplay);
    fn on_event(&mut self, event: &AppEvent, services: &mut Context) -> Option<RouterCommand>;
}
