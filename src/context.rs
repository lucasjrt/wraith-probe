use embedded_graphics::primitives::Rectangle;

pub struct Context {
    services: Services,
    dirty: Option<Dirty>,
}

pub struct Services {
    // Add service handles here as needed
    // wifi: Sender<WifiCommand>,
    // ui: Sender<UiCommand>,
}

pub enum Dirty {
    Full,
    Partial(Rectangle),
}

#[allow(dead_code)]
impl Context {
    pub fn new() -> Self {
        Context {
            services: Services {
                // wifi,
                // ui,
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
