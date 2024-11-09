//! Print a "Hello, World!" message to the debugger and blink the LED on GPIO1.

#![no_std]
#![no_main]
// Delete the following line after you're done implementing
// the solution.
#![allow(unused)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_time::{Timer, Duration};
use embassy_rp::gpio::{AnyPin, Input, Level, Output, Pin, Pull};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::task]
async fn blink_1(pin: AnyPin) {
    let mut led = Output::new(pin, Level::Low);

    loop
    {
        led.set_high();
        Timer::after_millis(150).await;
        led.set_low();
        Timer::after_millis(150).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    
    Timer::after(Duration::from_secs(5)).await;
    defmt::println!("Hello world!");

    spawner.spawn(blink_1(p.PIN_1.degrade())).unwrap();
}
