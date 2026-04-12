use std::sync::mpsc::Sender;

use embedded_graphics::primitives::Rectangle;
use esp_idf_svc::timer::EspTaskTimerService;

use crate::events::AppEvent;

pub struct Context {
    services: Services,
    dirty: Option<Dirty>,
}

pub struct Services {
    timer: EspTaskTimerService,
    app: Sender<AppEvent>,
}

#[derive(Debug, Clone)]
pub enum Dirty {
    Full,
    Partial(Rectangle),
}

impl Context {
    pub fn new(app: Sender<AppEvent>) -> Self {
        Context {
            services: Services {
                timer: EspTaskTimerService::new().expect("Failed to create timer service"),
                app,
            },
            dirty: Some(Dirty::Full),
        }
    }

    pub fn set_dirty(&mut self, dirty: Dirty) {
        self.dirty = Some(dirty);
    }

    pub fn clear_dirty(&mut self) {
        self.dirty = None;
    }

    pub fn get_dirty(&self) -> &Option<Dirty> {
        &self.dirty
    }

    pub fn services(&self) -> &Services {
        &self.services
    }
}

impl Services {
    pub fn timer(&self) -> &EspTaskTimerService {
        &self.timer
    }

    pub fn app(&self) -> &Sender<AppEvent> {
        &self.app
    }
}
