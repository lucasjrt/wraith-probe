use button_driver::{Button, ButtonConfig};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop, hal::{delay::Delay, gpio::PinDriver, modem::Modem, prelude::Peripherals}, nvs::EspDefaultNvsPartition, sys::EspError, wifi::{AccessPointInfo, BlockingWifi, ClientConfiguration, Configuration, EspWifi}
};
use std::{sync::mpsc, thread, time::Instant};

mod display;

enum ScanCommand {
    Start,
    Stop,
}

enum ScanState {
    Idle,
    Scanning,
    Complete(Vec<AccessPointInfo>),
    Failed,
}

fn init_sys() {
    esp_idf_svc::log::EspLogger::initialize_default();
    esp_idf_svc::sys::link_patches();
}

fn init_wifi<'a>(modem: Modem) -> Result<Box<BlockingWifi<EspWifi<'a>>>, EspError> {
    log::info!("Starting wifi agent thread");
        log::info!("Getting event loop");
        let sysloop = EspSystemEventLoop::take().unwrap();
        let nvs = EspDefaultNvsPartition::take().unwrap();

        log::info!("Creating wifi");
        let raw_wifi = match EspWifi::new(modem, sysloop.clone(), Some(nvs)) {
            Ok(wifi) => {
                log::info!("Wifi created");
                Box::new(wifi)
            }
            Err(e) => {
                log::error!("Failed to create wifi: {:?}", e);
                return Err(e);
            }
        };

        log::info!("Setting wifi mode");
        let mut wifi = match BlockingWifi::wrap(*raw_wifi, sysloop) {
            Ok(wifi) => {
                log::info!("Wifi wrapped");
                Box::new(wifi)
            }
            Err(e) => {
                log::error!("Failed to wrap wifi: {:?}", e);
                return Err(e);
            }
        };

        log::info!("Setting wifi configuration to client mode");
        match wifi.set_configuration(&Configuration::Client(ClientConfiguration::default())) {
            Ok(_) => {
                log::info!("Wifi configuration set");
            }
            Err(e) => {
                log::error!("Failed to set wifi configuration: {:?}", e);
                return Err(e);
            }
        };

        log::info!("Starting wifi");
        match wifi.start() {
            Ok(_) => {
                log::info!("Wifi started");
            }
            Err(e) => {
                log::error!("Failed to start wifi: {:?}", e);
                return Err(e);
            }
        };
        Ok(wifi)
    }

fn main() -> Result<(), EspError> {
    init_sys();

    log::info!("ESP32 initialized!");

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;
    let modem = peripherals.modem;

    let select_button_pin = PinDriver::input(pins.gpio0).unwrap();
    let mut select_button = Button::<_, Instant>::new(select_button_pin, ButtonConfig::default());

    let back_button_pin = PinDriver::input(pins.gpio35).unwrap();
    let mut back_button = Button::<_, Instant>::new(back_button_pin, ButtonConfig::default());

    log::info!("Buttons initialized!");

    let display = display::EspDisplay::new(
        pins.gpio23,
        pins.gpio16,
        peripherals.spi2,
        pins.gpio18,
        pins.gpio19,
        pins.gpio5,
        pins.gpio4,
    );
    display.lock().unwrap().enable_backlight();
    log::info!("Display initialized!");

    display.lock().unwrap().fill(None);

    let delay: Delay = Delay::new(1);

    let (cmd_tx, cmd_rx) = mpsc::channel::<ScanCommand>();
    let (state_tx, state_rx) = mpsc::channel::<ScanState>();

    log::info!("Starting wifi agent");
    display.lock().unwrap().text("Starting wifi agent", 10, 10);
    display.lock().unwrap().text("Please wait...", 10, 30);
    delay.delay_ms(2000);

    let mut wifi = match init_wifi(modem) {
        Ok(wifi) => {
            log::info!("Wifi agent initialized");
            display.lock().unwrap().text("Wifi agent initialized", 10, 10);
            wifi
        }
        Err(e) => {
            log::error!("Failed to initialize wifi agent: {:?}", e);
            display.lock().unwrap().text("Failed to initialize wifi agent", 10, 10);
            return Err(e);
        }
    };

    // Wifi agent
    thread::Builder::new().stack_size(32 * 1024).spawn(move || {
        log::info!("Agent init complete. Waiting for commands...");
        for cmd in cmd_rx {
            match cmd {
                ScanCommand::Start => {
                    let _ = state_tx.send(ScanState::Scanning);
                    log::info!("Starting scan");
                    match wifi.scan() {
                        Ok(aps) => {
                            log::info!("Scan complete: {:?}", aps);
                            state_tx.send(ScanState::Complete(aps)).unwrap();
                        }
                        Err(e) => {
                            log::error!("Failed to start scan: {:?}", e);
                            state_tx.send(ScanState::Failed).unwrap();
                            continue;
                        }
                    };
                    log::info!("Scan complete");
                }
                ScanCommand::Stop => {
                    log::info!("Stopping scan");
                    wifi.stop().unwrap();
                    state_tx.send(ScanState::Idle).unwrap();
                }
            }
        }
    }).unwrap();

    log::info!("Post wifi agent thread");
    display.lock().unwrap().clear();
    display.lock().unwrap().text("Wraith probe", 10, 10);
    display.lock().unwrap().text("Press Select to scan", 10, 30);

    loop {
        select_button.tick();
        back_button.tick();

        if select_button.is_clicked() {
            log::info!("Starting wifi scan");
            cmd_tx.send(ScanCommand::Start).unwrap();
        }

        if back_button.is_clicked() {
            log::info!("Stopping wifi scan");
            cmd_tx.send(ScanCommand::Stop).unwrap();
            display.lock().unwrap().clear();
            display.lock().unwrap().text("Scan stopped", 10, 10);
            delay.delay_ms(1000);
            display.lock().unwrap().clear();
            display.lock().unwrap().text("Press Select to scan", 10, 10);
            continue;
        }

        while let Ok(state) = state_rx.try_recv() {
            match state {
                ScanState::Scanning => {
                    display.lock().unwrap().clear();
                    display.lock().unwrap().text("Scanning...", 10, 10);
                }
                ScanState::Complete(aps) => {
                    display.lock().unwrap().clear();
                    for (i, ap) in aps.iter().enumerate() {
                        if i >= 10 {
                            break;
                        }
                        display.lock().unwrap().text(&ap.ssid, 0, 10 + ((i as i32) * 10));
                    }
                }
                ScanState::Failed => {
                    display.lock().unwrap().text("Scan failed", 0, 0);
                }
                ScanState::Idle => {}
            }
        }

        select_button.reset();
        back_button.reset();
        delay.delay_us(10);
    }
}
