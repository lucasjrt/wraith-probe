use crate::{context::Context, events::AppEvent, ui::router::RouterCommand};

pub trait Menu {
    fn selected(&self) -> usize;
    fn previous_selected(&self) -> usize;
    fn items(&self) -> &[&str];
}
