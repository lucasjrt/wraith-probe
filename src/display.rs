use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::{IntoStorage, Point, RgbColor},
    text::Text,
    Drawable,
};
use esp_idf_svc::hal::{
    delay::Ets,
    gpio::{AnyInputPin, Gpio16, Gpio23, Gpio4, Output, PinDriver},
    prelude::Peripherals,
    spi::{
        config::{Config as DeviceConfig, DriverConfig},
        Dma, SpiDeviceDriver, SpiDriver,
    },
    units::Hertz,
};
use mipidsi::{interface::SpiInterface, models::ST7789, Builder, Display};

pub struct EspDisplay<'a, 'b> {
    display: DisplayType<'a, 'b>,
    backlight: PinDriver<'a, Gpio4, Output>,
    pixels: &'a mut [u16; ESP_DISPLAY_BUFFER_SIZE],
}

type DisplayType<'a, 'b> = Display<
    SpiInterface<'a, SpiDeviceDriver<'b, SpiDriver<'b>>, PinDriver<'a, Gpio16, Output>>,
    ST7789,
    PinDriver<'a, Gpio23, esp_idf_svc::hal::gpio::Output>,
>;
pub const ESP_DISPLAY_WIDTH: u16 = 135;
pub const ESP_DISPLAY_HEIGHT: u16 = 240;
pub const ESP_DISPLAY_BUFFER_SIZE: usize =
    (ESP_DISPLAY_WIDTH as usize) * (ESP_DISPLAY_HEIGHT as usize);

impl<'a, 'b> EspDisplay<'a, 'b> {
    pub fn new() -> Self {
        let peripherals = Peripherals::take().unwrap();
        let pins = peripherals.pins;

        let rst = PinDriver::output(pins.gpio23).unwrap();

        let dc = PinDriver::output(pins.gpio16).unwrap();
        let driver_cfg = DriverConfig::new().dma(Dma::Auto(4096));
        let spi = SpiDriver::new(
            peripherals.spi2,
            pins.gpio18,
            pins.gpio19,
            None::<AnyInputPin>,
            &driver_cfg,
        )
        .unwrap();

        let dev_cfg = DeviceConfig::new()
            .queue_size(1)
            .baudrate(Hertz(32_500_000));
        let spi_device = SpiDeviceDriver::new(spi, Some(pins.gpio5), &dev_cfg).unwrap();

        let boxed_scratch_buffer = Box::new([0u8; 512]);
        let scratch_buffer = Box::leak(boxed_scratch_buffer);
        let di = SpiInterface::new(spi_device, dc, &mut *scratch_buffer);

        let mut delay = Ets;
        let display = Builder::new(ST7789, di)
            .display_size(135, 240)
            .reset_pin(rst)
            // On the TTGO T-Display, the ST7789 controller’s on-chip “frame memory” is larger (240 × 320 pixels)
            // than the little 135 × 240 window that the glass actually shows. In other words, the chip thinks it
            // has a 240 × 320 canvas, and you only see a 135 × 240 slice of it.
            // That's why we need to set the display offset to 52, 40.
            .display_offset(52, 40)
            .invert_colors(mipidsi::options::ColorInversion::Inverted)
            .init(&mut delay)
            .unwrap();

        let boxed_pixels = Box::new([0u16; ESP_DISPLAY_BUFFER_SIZE]);
        let pixels = Box::leak(boxed_pixels);

        let mut backlight = PinDriver::output(pins.gpio4).unwrap();
        backlight.set_high().unwrap();

        EspDisplay {
            display,
            backlight,
            pixels,
        }
    }

    #[allow(dead_code)]
    pub fn get_display(&mut self) -> &mut DisplayType<'a, 'b> {
        &mut self.display
    }

    #[allow(dead_code)]
    pub fn toggle_backlight(&mut self) {
        self.backlight.toggle().unwrap();
    }

    pub fn hello(&mut self) {
        let style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
        Text::new("Hello, World!", Point::new(10, 10), style)
            .draw(&mut self.display)
            .unwrap();
    }

    pub fn fill(&mut self, color: Option<Rgb565>) {
        let color = color.unwrap_or(Rgb565::BLACK);
        self.pixels.fill(color.into_storage());
        self.display
            .set_pixels(
                0,
                0,
                ESP_DISPLAY_WIDTH - 1,
                ESP_DISPLAY_HEIGHT - 1,
                self.pixels.iter().copied().map(|u: u16| {
                    Rgb565::new((u >> 11) as u8, ((u >> 5) & 0x3F) as u8, (u & 0x1F) as u8)
                }),
            )
            .unwrap();
    }

    pub fn enable_backlight(&mut self) {
        self.backlight.set_high().unwrap();
    }

    #[allow(dead_code)]
    pub fn disable_backlight(&mut self) {
        self.backlight.set_low().unwrap();
    }
}
