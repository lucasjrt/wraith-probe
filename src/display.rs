use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::{IntoStorage, Point, RgbColor},
    text::Text,
    Drawable,
};
use esp_idf_svc::hal::{
    delay::Ets,
    gpio::{AnyInputPin, Gpio16, Gpio18, Gpio19, Gpio23, Gpio4, Gpio5, Output, PinDriver},
    spi::{
        config::{Config as DeviceConfig, DriverConfig},
        Dma, SpiDeviceDriver, SpiDriver,
    },
    units::Hertz,
};
use mipidsi::{interface::SpiInterface, models::ST7789, options::Orientation, Builder, Display};

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
pub const ESP_DISPLAY_WIDTH: u16 = 240;
pub const ESP_DISPLAY_HEIGHT: u16 = 135;
pub const ESP_DISPLAY_BUFFER_SIZE: usize =
    (ESP_DISPLAY_WIDTH as usize) * (ESP_DISPLAY_HEIGHT as usize);

impl<'a, 'b> EspDisplay<'a, 'b> {
    pub fn new(rst_pin: Gpio23,
                dc_pin: Gpio16,
                spi2: esp_idf_svc::hal::spi::SPI2,
                sclk_pin: Gpio18,
                sdo_pin: Gpio19,
                cs_pin: Gpio5,
                backlight_pin: Gpio4) -> Self {
        let rst = PinDriver::output(rst_pin).unwrap();

        let dc = PinDriver::output(dc_pin).unwrap();
        let driver_cfg = DriverConfig::new().dma(Dma::Auto(65536));
        let spi = SpiDriver::new(
            spi2,
            sclk_pin,
            sdo_pin,
            None::<AnyInputPin>,
            &driver_cfg,
        )
        .unwrap();

        let dev_cfg = DeviceConfig::new()
            .queue_size(1)
            .baudrate(Hertz(30*1024*1024));
        let spi_device = SpiDeviceDriver::new(spi, Some(cs_pin), &dev_cfg).unwrap();

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
            .orientation(Orientation::new().rotate(mipidsi::options::Rotation::Deg90))
            .init(&mut delay)
            .unwrap();

        let boxed_pixels = Box::new([0u16; ESP_DISPLAY_BUFFER_SIZE]);
        let pixels = Box::leak(boxed_pixels);

        let mut backlight = PinDriver::output(backlight_pin).unwrap();
        backlight.set_high().unwrap();

        EspDisplay {
            display,
            backlight,
            pixels,
        }
    }

    #[allow(dead_code)]
    pub fn toggle_backlight(&mut self) {
        self.backlight.toggle().unwrap();
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

    pub fn text(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        color: Rgb565,
    ) {
        let style = MonoTextStyle::new(&FONT_6X10, color);
        Text::new(text, Point::new(x, y), style)
            .draw(&mut self.display)
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
