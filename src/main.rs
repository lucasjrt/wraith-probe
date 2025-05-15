use embedded_graphics::{pixelcolor::Rgb565, prelude::RgbColor};
use esp_idf_svc::hal::delay::Delay;

mod display;

fn init_sys() {
    esp_idf_svc::log::EspLogger::initialize_default();
    esp_idf_svc::sys::link_patches();
}

fn main() {
    init_sys();
    log::info!("ESP32 initialized!");

    let mut display = display::EspDisplay::new();
    display.enable_backlight();
    log::info!("Display initialized!");

    display.fill(None);
    display.hello();
    
    
    let colors = [
        Rgb565::RED,
        Rgb565::GREEN,
        Rgb565::BLUE,
        Rgb565::BLACK,
        Rgb565::YELLOW,
    ];

    let mut color = 0;
    let delay_seconds = 1;

    loop {
        log::info!("Color: {:?}", colors[color]);
       
        let delay: Delay = Default::default();
        delay.delay_ms(delay_seconds * 1000);
       
        display.fill(Some(colors[color]));
        display.hello();
        
        color += 1;
        if color >= colors.len() {
            color = 0;
        }
    }
}
