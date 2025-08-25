use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{ascii::FONT_8X13, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::{Point, PointsIter, RgbColor, Size},
    primitives::Rectangle,
    text::Text,
    Drawable,
};
use esp_idf_svc::hal::{
    delay::Ets,
    gpio::{AnyInputPin, Gpio16, Gpio18, Gpio19, Gpio23, Gpio4, Gpio5, Output, PinDriver},
    spi::{
        config::{Config as DeviceConfig, DriverConfig, Duplex, MODE_0},
        Dma, SpiDeviceDriver, SpiDriver,
    },
    units::Hertz,
};
use mipidsi::{interface::SpiInterface, models::ST7789, options::Orientation, Builder, Display};

#[allow(dead_code)]
pub struct EspDisplay {
    display: DisplayType,
    backlight: PinDriver<'static, Gpio4, Output>,
    pixels: Box<[u16; ESP_DISPLAY_BUFFER_SIZE]>,
}

type DisplayType = Display<
    SpiInterface<
        'static,
        SpiDeviceDriver<'static, SpiDriver<'static>>,
        PinDriver<'static, Gpio16, Output>,
    >,
    ST7789,
    PinDriver<'static, Gpio23, esp_idf_svc::hal::gpio::Output>,
>;
pub const ESP_DISPLAY_WIDTH: u16 = 240;
pub const ESP_DISPLAY_HEIGHT: u16 = 135;
pub const ESP_DISPLAY_BUFFER_SIZE: usize =
    (ESP_DISPLAY_WIDTH as usize) * (ESP_DISPLAY_HEIGHT as usize);
pub const FONT_COLOR: Rgb565 = Rgb565::GREEN;
pub const BACKGROUND_COLOR: Rgb565 = Rgb565::BLACK;

impl EspDisplay {
    pub fn new(
        rst_pin: Gpio23,
        dc_pin: Gpio16,
        spi2: esp_idf_svc::hal::spi::SPI2,
        sclk_pin: Gpio18,
        sdo_pin: Gpio19,
        cs_pin: Gpio5,
        backlight_pin: Gpio4,
    ) -> Self {
        // ) {
        let rst = PinDriver::output(rst_pin).unwrap();
        let dc = PinDriver::output(dc_pin).unwrap();

        let driver_cfg = DriverConfig::new().dma(Dma::Auto(65_536));
        let spi =
            SpiDriver::new(spi2, sclk_pin, sdo_pin, None::<AnyInputPin>, &driver_cfg).unwrap();

        let dev_cfg = DeviceConfig::new()
            .queue_size(10)
            .baudrate(Hertz(40_000_000))
            .duplex(Duplex::Half)
            .data_mode(MODE_0);
        let spi_device = SpiDeviceDriver::new(spi, Some(cs_pin), &dev_cfg).unwrap();

        let boxed_scratch_buffer = Box::new([0u8; 64 * 1024]);
        let scratch_buffer = Box::leak(boxed_scratch_buffer);
        let di = SpiInterface::new(spi_device, dc, scratch_buffer);

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

        let pixels = Box::new([0u16; ESP_DISPLAY_BUFFER_SIZE]);

        let mut backlight = PinDriver::output(backlight_pin).unwrap();
        backlight.set_high().unwrap();

        Self {
            display,
            backlight,
            pixels,
        }
    }

    #[allow(dead_code)]
    pub fn toggle_backlight(&mut self) {
        self.backlight.toggle().unwrap();
    }

    pub fn clear(&mut self) {
        let area = Rectangle {
            top_left: Point::new(0, 0),
            size: Size::new(ESP_DISPLAY_WIDTH as u32, ESP_DISPLAY_HEIGHT as u32),
        };
        self.display
            .fill_contiguous(&area, area.points().map(|_| BACKGROUND_COLOR))
            .unwrap();
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: Option<Rgb565>) {
        let fill_color = color.unwrap_or(BACKGROUND_COLOR);
        let area = Rectangle {
            top_left: Point::new(x, y),
            size: Size::new(w as u32, h as u32),
        };
        self.display
            .fill_contiguous(&area, area.points().map(|_| fill_color))
            .unwrap();
    }

    pub fn text(&mut self, text: &str, x: i32, y: i32) {
        let style = MonoTextStyle::new(&FONT_8X13, FONT_COLOR);
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
