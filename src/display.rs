use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{ascii::FONT_8X13, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::{Dimensions, Point, PointsIter, RgbColor, Size},
    primitives::Rectangle,
    text::{Alignment, Baseline, Text, TextStyleBuilder},
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

pub struct EspDisplay {
    display: DisplayType,
    backlight: PinDriver<'static, AnyIOPin, Output>,
    pixels: Box<[Rgb565; ESP_DISPLAY_BUFFER_SIZE]>,
    theme: Theme,
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
pub const ESP_DISPLAY_WIDTH: usize = 240;
pub const ESP_DISPLAY_HEIGHT: usize = 135;
pub const ESP_DISPLAY_BUFFER_SIZE: usize =
    (ESP_DISPLAY_WIDTH as usize) * (ESP_DISPLAY_HEIGHT as usize);

pub struct Theme {
    pub primary: Rgb565,
    pub secondary: Rgb565,
}

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
            theme: Theme::new(Rgb565::GREEN, Rgb565::BLACK),
        }
    }

    pub fn toggle_backlight(&mut self) {
        self.backlight.toggle().unwrap();
    }

    pub fn clear(&mut self) {
        self.fill_rect(
            0,
            0,
            ESP_DISPLAY_WIDTH as i32,
            ESP_DISPLAY_HEIGHT as i32,
            Some(self.theme().secondary()),
        );
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: Option<Rgb565>) {
        let color = color.unwrap_or(self.theme().secondary());
        let area = Rectangle::new(Point::new(x, y), Size::new(w as u32, h as u32));
        self.draw_iter(area.points().map(|p| Pixel(p, color)))
            .unwrap();
    }

    pub fn text(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        color: Option<Rgb565>,
        alignment: Option<Alignment>,
    ) {
        let alignment = alignment.unwrap_or(Alignment::Left);
        let color = color.unwrap_or(self.theme().primary());
        let font = MonoTextStyle::new(&FONT_8X13, color);
        let style = TextStyleBuilder::new()
            .baseline(Baseline::Top)
            .alignment(alignment)
            .build();
        let mut x = x;

        if alignment == Alignment::Center && x < 0 {
            x = ESP_DISPLAY_WIDTH as i32 / 2;
        }

        Text::with_text_style(text, Point::new(x, y), font, style)
            .draw(self)
            .unwrap();
    }

    pub fn enable_backlight(&mut self) {
        self.backlight.set_high().unwrap();
    }

    pub fn disable_backlight(&mut self) {
        self.backlight.set_low().unwrap();
    }

    pub fn flush(&mut self) {
        self.display
            .fill_contiguous(&self.display.bounding_box(), self.pixels.iter().copied())
            .unwrap();
    }

    pub fn font_height(&self) -> usize {
        // TODO: Make this dynamic based on the font used
        13
    }

    pub fn font_width(&self) -> usize {
        // TODO: Make this dynamic based on the font used
        8
    }

    pub fn width(&self) -> usize {
        ESP_DISPLAY_WIDTH
    }

    pub fn height(&self) -> usize {
        ESP_DISPLAY_HEIGHT
    }

    pub fn theme(&self) -> &Theme {
        &self.theme
    }
}

impl Theme {
    pub fn new(primary: Rgb565, secondary: Rgb565) -> Self {
        Self { primary, secondary }
    }

    pub fn primary(&self) -> Rgb565 {
        self.primary
    }

    pub fn secondary(&self) -> Rgb565 {
        self.secondary
    }
}

impl DrawTarget for EspDisplay {
    type Color = Rgb565;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
    {
        let width: i32 = ESP_DISPLAY_WIDTH as i32;
        let height: i32 = ESP_DISPLAY_HEIGHT as i32;
        for Pixel(coord, color) in pixels.into_iter() {
            let (x, y) = coord.into();
            if x >= 0 && x < width && y >= 0 && y < height {
                let index = (y * width + x) as usize;
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
