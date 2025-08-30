use esp_idf_svc::{
    hal::{delay::Delay, gpio::PinDriver, prelude::Peripherals},
    sys::EspError,
};
use std::{sync::mpsc, thread};

mod context;
mod display;
mod events;
mod input;
mod menu;
mod ui;
// mod wifi;

use events::AppEvent;
use ui::router::Router;
// use wifi::init_wifi;

use crate::{
    context::{Context, Dirty},
    display::EspDisplay,
    ui::screens::main::MainMenu,
};

fn init_sys() {
    esp_idf_svc::log::EspLogger::initialize_default();
    esp_idf_svc::sys::link_patches();
}

fn main() -> Result<(), EspError> {
    log::info!("Starting ESP32 application");
    init_sys();

    log::info!("ESP32 initialized!");

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;
    // let modem = peripherals.modem;

    let select_button_pin = PinDriver::input(pins.gpio0).unwrap();
    let back_button_pin = PinDriver::input(pins.gpio35).unwrap();

    let (app_tx, app_rx) = mpsc::channel::<AppEvent>();

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

    log::info!("Starting input handler");
    let mut input = input::Input::new(select_button_pin, back_button_pin);
    thread::spawn(move || loop {
        input.run(app_tx.clone());
    });
    log::info!("Input handler started");

    // log::info!("Starting wifi agent");
    // display.text("Starting wifi agent", 10, 10);
    // display.text("Please wait...", 10, 30);
    //
    // let mut _wifi = match init_wifi(modem) {
    //     Ok(wifi) => {
    //         display.clear();
    //         display.text("Wifi agent initialized", 13, 13);
    //         wifi
    //     }
    //     Err(e) => {
    //         log::error!("Failed to initialize wifi agent: {:?}", e);
    //         display.clear();
    //         display.text("Failed to initialize wifi agent", 10, 10);
    //         return Err(e);
    //     }
    // };
    // log::info!("Wifi agent started");

    log::info!("Initialization complete, starting ui");
    display.text("Initialization complete", 10, 50, None, None);

    let delay = Delay::new(1);
    delay.delay_ms(1000);
    display.clear();

    // // Wifi agent
    // thread::Builder::new()
    //     .stack_size(32 * 1024)
    //     .spawn(move || {
    //         for cmd in wifi_agent_cmd_rx {
    //             match cmd {
    //                 ScanCommand::Start => {
    //                     let _ = wifi_agent_state_tx.send(ScanState::Scanning);
    //
    //                     match wifi.scan() {
    //                         Ok(aps) => {
    //                             log::info!("Scan complete: {:?}", aps);
    //                             wifi_agent_state_tx.send(ScanState::Complete(aps)).unwrap();
    //                         }
    //                         Err(e) => {
    //                             log::error!("Failed to start scan: {:?}", e);
    //                             wifi_agent_state_tx.send(ScanState::Failed).unwrap();
    //                             continue;
    //                         }
    //                     };
    //                     log::info!("Scan complete");
    //                 }
    //                 ScanCommand::Stop => {
    //                     log::info!("Stopping scan");
    //                     match wifi.wifi_mut().stop_scan() {
    //                         Ok(_) => {
    //                             log::info!("Scan stopped");
    //                         }
    //                         Err(e) => {
    //                             log::error!("Failed to stop scan: {:?}", e);
    //                             wifi_agent_state_tx.send(ScanState::Failed).unwrap();
    //                         }
    //                     };
    //                     wifi_agent_state_tx.send(ScanState::Idle).unwrap();
    //                 }
    //             }
    //         }
    //     })
    //     .unwrap();

    let mut router = Router::new(MainMenu::new());
    let mut ctx = Context::new();
    loop {
        if let Some(dirty_type) = ctx.get_dirty() {
            match dirty_type {
                Dirty::Full => {
                    log::info!("Full dirty context, re-rendering entire display");
                    display.clear();
                }
                Dirty::Partial(area) => {
                    // TODO: Implement partial rendering logic
                    log::info!("Partial dirty context, re-rendering area: {:?}", area);
                }
            }
            router.current_screen().render(&mut display);
            display.flush();
            ctx.clear_dirty();
        }

        if let Ok(event) = app_rx.recv() {
            log::info!("Event received: {:?}", event);
            if let Some(cmd) = router.on_event(&event, &mut ctx) {
                log::info!("Router command: {}", cmd);
                router.apply(cmd);
                ctx.set_dirty(Dirty::Full);
            }
        }
        delay.delay_us(10);
    }
}
