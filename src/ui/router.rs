use std::{
    fmt::Display,
    sync::{Arc, RwLock},
};

use crate::{context::Context, events::AppEvent, ui::screen::Screen};

pub enum RouterCommand {
    NavigateTo(Box<dyn Screen>),
    NavigateBack,
}

pub struct Router {
    current_screen: Box<dyn Screen>,
    menu_stack: Vec<Box<dyn Screen>>,
}

impl Display for RouterCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RouterCommand::NavigateTo(_) => write!(f, "NavigateTo(...)"),
            RouterCommand::NavigateBack => write!(f, "NavigateBack"),
        }
    }
}

impl Router {
    pub fn new<M: Screen + 'static>(screen: M) -> Self {
        Self {
            current_screen: Box::new(screen),
            menu_stack: Vec::new(),
        }
    }

    pub fn navigate_to(&mut self, menu: Box<dyn Screen>) {
        let old_menu = std::mem::replace(&mut self.current_screen, menu);
        self.menu_stack.push(old_menu);
    }

    pub fn navigate_back(&mut self) {
        if let Some(previous_menu) = self.menu_stack.pop() {
            self.current_screen = previous_menu;
        }
    }

    pub fn on_event(
        &mut self,
        event: &AppEvent,
        ctx: Arc<RwLock<Context>>,
    ) -> Option<RouterCommand> {
        self.current_screen_mut().on_event(event, ctx)
    }

    pub fn current_screen_mut(&mut self) -> &mut dyn Screen {
        &mut *self.current_screen
    }

    pub fn apply(&mut self, command: RouterCommand) {
        match command {
            RouterCommand::NavigateTo(menu) => self.navigate_to(menu),
            RouterCommand::NavigateBack => self.navigate_back(),
        }
    }
}
