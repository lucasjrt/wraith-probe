use esp_idf_svc::{
    hal::{delay::Delay, gpio::PinDriver, prelude::Peripherals},
    sys::EspError,
};
use std::{
    sync::{mpsc::channel, Arc, RwLock},
    thread,
};

mod context;
mod display;
mod events;
mod input;
mod menu;
mod ui;

use events::AppEvent;
use ui::router::Router;

use crate::{
    context::{Context, Dirty},
    display::EspDisplay,
    ui::screens::{booting::BootingScreen, main::MainMenu},
};

fn init_sys() {
    esp_idf_svc::log::EspLogger::initialize_default();
    esp_idf_svc::sys::link_patches();
}

fn main() -> Result<(), EspError> {
    log::info!("Starting Wraith Probe");
    init_sys();

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    let select_button_pin = PinDriver::input(pins.gpio0).unwrap();
    let back_button_pin = PinDriver::input(pins.gpio35).unwrap();

    let (app_tx, app_rx) = channel::<AppEvent>();
    let ctx = Arc::new(RwLock::new(Context::new(app_tx.clone())));

    log::info!("Initializing display");
    let mut display = EspDisplay::new(
        pins.gpio23.into(),
        pins.gpio16.into(),
        peripherals.spi2,
        pins.gpio18.into(),
        pins.gpio19.into(),
        pins.gpio5.into(),
        pins.gpio4.into(),
    );

    display.enable_backlight();
    display.clear();
    log::info!("Display initialized");

    let mut booting_screen = BootingScreen::new();
    booting_screen.log("Display initialized", &mut display, ctx.clone());

    booting_screen.log("Starting input handler", &mut display, ctx.clone());
    log::info!("Starting input handler");
    let mut input = input::Input::new(select_button_pin, back_button_pin);
    thread::spawn(move || {
        input.run(app_tx.clone());
    });
    log::info!("Input handler started");
    booting_screen.log("Input handler started", &mut display, ctx.clone());

    log::info!("Initialization complete, starting ui");
    booting_screen.log("Initialization complete", &mut display, ctx.clone());

    let delay = Delay::new(1);
    delay.delay_ms(1000);
    display.clear();

    log::info!("Initializing router with main menu");
    let mut router = Router::new(MainMenu::new());
    {
        ctx.write()
            .expect("Failed to lock context")
            .set_dirty(Dirty::Full);
    }
    loop {
        let dirty = {
            ctx.read()
                .expect("Failed to lock context")
                .get_dirty()
                .clone()
        };

        if let Some(dirty_type) = dirty {
            match dirty_type {
                Dirty::Full => {
                    display.clear();
                }
                Dirty::Partial(_area) => {
                    // TODO: Implement partial rendering logic
                }
            }
            router
                .current_screen_mut()
                .render(&mut display, ctx.clone());
            display.flush();
            {
                ctx.write().expect("Failed to lock context").clear_dirty();
            }
        }

        if let Ok(event) = app_rx.recv() {
            if event == AppEvent::Redraw {
                {
                    ctx.write()
                        .expect("Failed to lock context")
                        .set_dirty(Dirty::Full);
                }
                continue;
            }
            if let Some(cmd) = router.on_event(&event, ctx.clone()) {
                router.apply(cmd);
                {
                    ctx.write()
                        .expect("Failed to lock context")
                        .set_dirty(Dirty::Full);
                }
            }
        }
        delay.delay_us(10);
    }
}
