use std::time::Instant;

use button_driver::{Button, ButtonConfig};
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::RgbColor,
};
use esp_idf_svc::hal::{delay::Delay, gpio::PinDriver, prelude::Peripherals};

mod display;

fn init_sys() {
    esp_idf_svc::log::EspLogger::initialize_default();
    esp_idf_svc::sys::link_patches();
}

fn main() {
    init_sys();
    log::info!("ESP32 initialized!");

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    let select_button_pin = PinDriver::input(pins.gpio0).unwrap();
    let mut select_button = Button::<_, Instant>::new(select_button_pin, ButtonConfig::default());

    let back_button_pin = PinDriver::input(pins.gpio35).unwrap();
    let mut back_button = Button::<_, Instant>::new(back_button_pin, ButtonConfig::default());

    log::info!("Buttons initialized!");

    let mut display = display::EspDisplay::new(
        pins.gpio23,
        pins.gpio16,
        peripherals.spi2,
        pins.gpio18,
        pins.gpio19,
        pins.gpio5,
        pins.gpio4,
    );
    display.enable_backlight();
    log::info!("Display initialized!");

    display.fill(None);

    let colors = [
        Rgb565::RED,
        Rgb565::GREEN,
        Rgb565::BLUE,
        Rgb565::BLACK,
        Rgb565::YELLOW,
    ];

    let mut color = 0;

    let delay: Delay = Delay::new(1);

    loop {
        select_button.tick();
        back_button.tick();
        if select_button.is_clicked() {
            log::info!("Select button clicked");
            color = (color + 1) % colors.len();
            display.fill(Some(colors[color]));
            display.text(&format!("Color: {:?}", colors[color]), 10, 10, Rgb565::WHITE);
        }

        if back_button.is_clicked() {
            log::info!("Back button clicked");
            color = (color + colors.len() - 1) % colors.len();
        }

        if select_button.is_double_clicked() {
            display.text("Double Clicked!", 10, 20, Rgb565::WHITE);
            log::info!("Select button double clicked");
        }
        if back_button.is_double_clicked() {
            display.text("Double Clicked!", 10, 30, Rgb565::WHITE);
            log::info!("Back button double clicked");
        }
        if select_button.is_triple_clicked() {
            display.text("Triple Clicked!", 10, 40, Rgb565::WHITE);
            log::info!("Select button triple clicked");
        }

        if back_button.is_triple_clicked() {
            display.text("Triple Clicked!", 10, 50, Rgb565::WHITE);
            log::info!("Back button triple clicked");
        }

        select_button.reset();
        back_button.reset();

        delay.delay_us(10);
    }
}
