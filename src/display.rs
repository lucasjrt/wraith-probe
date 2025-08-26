use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{ascii::FONT_8X13, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::{Dimensions, Point, PointsIter, RgbColor, Size},
    primitives::Rectangle,
    text::Text,
    Drawable, Pixel,
};
use esp_idf_svc::hal::{
    delay::Ets,
    gpio::{AnyIOPin, AnyInputPin, Output, PinDriver},
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
    backlight: PinDriver<'static, AnyIOPin, Output>,
    pixels: Box<[Rgb565; ESP_DISPLAY_BUFFER_SIZE]>,
}

type DisplayType = Display<
    SpiInterface<
        'static,
        SpiDeviceDriver<'static, SpiDriver<'static>>,
        PinDriver<'static, AnyIOPin, Output>,
    >,
    ST7789,
    PinDriver<'static, AnyIOPin, esp_idf_svc::hal::gpio::Output>,
>;
pub const ESP_DISPLAY_WIDTH: i32 = 240;
pub const ESP_DISPLAY_HEIGHT: i32 = 135;
pub const ESP_DISPLAY_BUFFER_SIZE: usize =
    (ESP_DISPLAY_WIDTH as usize) * (ESP_DISPLAY_HEIGHT as usize);
pub const FONT_COLOR: Rgb565 = Rgb565::GREEN;
pub const BACKGROUND_COLOR: Rgb565 = Rgb565::BLACK;

impl EspDisplay {
    pub fn new(
        rst_pin: AnyIOPin,
        dc_pin: AnyIOPin,
        spi2: esp_idf_svc::hal::spi::SPI2,
        sclk_pin: AnyIOPin,
        sdo_pin: AnyIOPin,
        cs_pin: AnyIOPin,
        backlight_pin: AnyIOPin,
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

        let pixels = Box::new([Rgb565::BLACK; ESP_DISPLAY_BUFFER_SIZE]);

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
        self.fill_rect(
            0,
            0,
            ESP_DISPLAY_WIDTH,
            ESP_DISPLAY_HEIGHT,
            Some(BACKGROUND_COLOR),
        );
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: Option<Rgb565>) {
        let color = color.unwrap_or(BACKGROUND_COLOR);
        let area = Rectangle::new(Point::new(x, y), Size::new(w as u32, h as u32));
        self.draw_iter(area.points().map(|p| Pixel(p, color)))
            .unwrap();
    }

    pub fn text(&mut self, text: &str, x: i32, y: i32) {
        let style = MonoTextStyle::new(&FONT_8X13, FONT_COLOR);
        Text::new(text, Point::new(x, y), style).draw(self).unwrap();
    }

    pub fn enable_backlight(&mut self) {
        self.backlight.set_high().unwrap();
    }

    #[allow(dead_code)]
    pub fn disable_backlight(&mut self) {
        self.backlight.set_low().unwrap();
    }

    pub fn flush(&mut self) {
        self.display
            .fill_contiguous(&self.display.bounding_box(), self.pixels.iter().copied())
            .unwrap();
    }
}

impl DrawTarget for EspDisplay {
    type Color = Rgb565;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            if let (x @ 0..ESP_DISPLAY_WIDTH, y @ 0..ESP_DISPLAY_HEIGHT) = coord.into() {
                let index = (y * ESP_DISPLAY_WIDTH + x) as usize;
                self.pixels[index] = color;
            }
        }
        Ok(())
    }
}

impl Dimensions for EspDisplay {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(
            Point::zero(),
            Size::new(ESP_DISPLAY_WIDTH as u32, ESP_DISPLAY_HEIGHT as u32),
        )
    }
}
