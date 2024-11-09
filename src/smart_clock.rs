//! Your Pico should act as the core MCU of a smart clock system.
//! Your device should have the following functionalities:
//!     * It will display the current time and temperature in the room.
//! In order to do that it will send a get request to the provided ip and port
//! to get the current time at the beginning of the runtime.
//!     * It will have provide a visual feedback of the current temperature
//! by displaying a color between red and blue on the RGB led, depending on
//! configurable maximum and minimum thresholds.
//!     * In order to update the thresholds, the desired behaviour is to
//! enter the configure mode by pressing the A button, then the current minimum
//! threshold value will be displayed on the screen, and by pressing X and Y,
//! the user should be able to increase and decrease respectively be half a
//! degree, then confirm it by pressing A once again, and proceed to setting
//! the maximum threshold in the same fashion.
//!     * To ensure redundency, the thresholds will be written in the provided
//! EEPROM24C256 when set, and read at the beginning of the program.
//!     * BONUS: We will simulate the fact that the clock is part of an evil
//! IoT network that spies on its users by sending a JSON package via HTTPS
//! to the same server, containing the datetime and the temperature.

#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::{Timer, Duration};
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Point;
use embedded_graphics::mono_font::iso_8859_1::FONT_7X13_BOLD;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::{Rgb565, RgbColor};
use embedded_graphics::text::Text;
use embedded_graphics::Drawable;
use embedded_nov_2024::display::SPIDeviceInterface;
use ds3231::Ds3231;
use {defmt_rtt as _, panic_probe as _};

const DISPLAY_FREQ: u32 = 64_000_000;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    info!("Initializing display...");

    // ************** Display initialization - DO NOT MODIFY! *****************
    let miso = peripherals.PIN_4;
    let display_cs = peripherals.PIN_17;
    let mosi = peripherals.PIN_19;
    let clk = peripherals.PIN_18;
    let rst = peripherals.PIN_0;
    let dc = peripherals.PIN_16;
    let mut display_config = embassy_rp::spi::Config::default();
    display_config.frequency = DISPLAY_FREQ;
    display_config.phase = embassy_rp::spi::Phase::CaptureOnSecondTransition;
    display_config.polarity = embassy_rp::spi::Polarity::IdleHigh;

    // Init SPI
    let spi: embassy_rp::spi::Spi<'_, _, embassy_rp::spi::Blocking> =
        embassy_rp::spi::Spi::new_blocking(
            peripherals.SPI0,
            clk,
            mosi,
            miso,
            display_config.clone(),
        );
    let spi_bus: embassy_sync::blocking_mutex::Mutex<
        embassy_sync::blocking_mutex::raw::NoopRawMutex,
        _,
    > = embassy_sync::blocking_mutex::Mutex::new(core::cell::RefCell::new(spi));

    let display_spi = embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig::new(
        &spi_bus,
        embassy_rp::gpio::Output::new(display_cs, embassy_rp::gpio::Level::High),
        display_config,
    );

    let dc = embassy_rp::gpio::Output::new(dc, embassy_rp::gpio::Level::Low);
    let rst = embassy_rp::gpio::Output::new(rst, embassy_rp::gpio::Level::Low);
    let di = SPIDeviceInterface::new(display_spi, dc);

    // Init ST7789 LCD
    let mut display = st7789::ST7789::new(di, rst, 240, 240);
    display.init(&mut embassy_time::Delay).unwrap();
    display
        .set_orientation(st7789::Orientation::Portrait)
        .unwrap();
    display.clear(<embedded_graphics::pixelcolor::Rgb565 as embedded_graphics::pixelcolor::RgbColor>::BLACK).unwrap();
    // ************************************************************************

    info!("Display initialization finished!");
    
    let i2c = I2c::new(I2C0, Config::default());
    let mut rtc = Ds3231::new(i2c);

    match rtc.get_time().await {
        Ok(time) => {
            info!("Current time: {}:{}:{}", time.hours, time.minutes, time.second);
        },
        Err(e) => {
            info!("Failed to read time: {:?}", e);
        }
    }
}
