use std::fmt::Display;

use crate::{context::Context, events::AppEvent, menu::Menu};

pub enum RouterCommand {
    NavigateTo(Box<dyn Menu>),
    NavigateBack,
}

pub struct Router {
    current_menu: Box<dyn Menu>,
    menu_stack: Vec<Box<dyn Menu>>,
}

impl Display for RouterCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RouterCommand::NavigateTo(_) => write!(f, "NavigateTo(...)"),
            RouterCommand::NavigateBack => write!(f, "NavigateBack"),
        }
    }
}

#[allow(dead_code)]
impl Router {
    pub fn new<M: Menu + 'static>(menu: M) -> Self {
        Self {
            current_menu: Box::new(menu),
            menu_stack: Vec::new(),
        }
    }

    pub fn navigate_to(&mut self, menu: Box<dyn Menu>) {
        let old_menu = std::mem::replace(&mut self.current_menu, menu);
        self.menu_stack.push(old_menu);
    }

    pub fn navigate_back(&mut self) {
        if let Some(previous_menu) = self.menu_stack.pop() {
            self.current_menu = previous_menu;
        }
    }

    pub fn current_menu(&self) -> &dyn Menu {
        &*self.current_menu
    }

    pub fn on_event(&mut self, event: &AppEvent, ctx: &mut Context) {
        self.current_menu_mut().on_event(event, ctx);
    }

    pub fn current_menu_mut(&mut self) -> &mut dyn Menu {
        &mut *self.current_menu
    }

    pub fn apply(&mut self, command: RouterCommand) {
        match command {
            RouterCommand::NavigateTo(menu) => self.navigate_to(menu),
            RouterCommand::NavigateBack => self.navigate_back(),
        }
    }
}
