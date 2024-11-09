//! Go to [Random Color Generator](https://randomwordgenerator.com/color.php)
//! Generate two colors and get the RGB encodings for them. These are the colors
//! you will need to display on the RGB LED.
//!
//! Your application should smoothly transition from one color to another. The colors will
//! be displayed sequentially for 3 seconds each, with a gradual transition period of 1 second.
//!
//! Keep in mind that the RGB LED is common anode (look at the figure in the README file).
//!
//! For displaying the color on the LED, PWM (Pulse Width Modulation) will need to be set up
//! on the pin. Connect them to pins: GPIO0 (Red), GPIO1 (Green), and
//! GPIO2 (Blue). (Hint: Pin 0 and 1 will share the same channel).

#![no_std]
#![no_main]
// Delete the following line after you're done implementing
// the solution.
#![allow(unused)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::peripherals::{PIN_0, PIN_1, PIN_2, PWM_SLICE0, PWM_SLICE1};
use embassy_rp::pwm::{Config as PwmConfig, Pwm, SetDutyCycle};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // TODO 0 : Create the Config for the PWM that will drive the RGB LED.

    // TODO 1 : Modify the RGB values and loop through the configs to create a transition.
}
