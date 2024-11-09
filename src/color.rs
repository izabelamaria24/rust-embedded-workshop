//! Go to [Random Color Generator](https://randomwordgenerator.com/color.php)
//! Generate two colors and get the RGB encodings for them. These are the colors
//! you will need to display on the RGB LED.
//!
//! Your application should smoothly transition from one color to another. The colors will
//! be displayed sequentially for 3 seconds each, with a gradual transition period of 1 second.
//!
//! Keep in mind that the RGB LED is common anode.
//!
//! For displaying the color on the LED, PWM (Pulse Width Modulation) will need to be set up
//! on the pin. Connect them to pins: GPIO0 (Red), GPIO1 (Green), and
//! GPIO2 (Blue). (Hint: Pin 0 and 1 will share the same channel).
 
#![no_std]
#![no_main]
// Delete the following line after you're done implementing
// the solution.
#![allow(unused)]
 
const COLOR1: (u8, u8, u8) = (0, 255, 0); // (green)
const COLOR2: (u8, u8, u8) = (0, 0, 255);  // (blue)
 
use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::peripherals::{PIN_0, PIN_1, PIN_2, PWM_SLICE0, PWM_SLICE1};
use embassy_rp::pwm::{Config as PwmConfig, Pwm, SetDutyCycle};
use embassy_time::*;
use {defmt_rtt as _, panic_probe as _};
 
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
 
    let mut pwm_blue_green = Pwm::new_output_ab(p.PWM_SLICE0, p.PIN_0, p.PIN_1, PwmConfig::default());
    let mut pwm_red = Pwm::new_output_a(p.PWM_SLICE1, p.PIN_2, PwmConfig::default());
 
    loop {
        transition_color(
            &mut pwm_blue_green,
            &mut pwm_red,
            COLOR1,
            COLOR2,
            Duration::from_secs(1),
        )
        .await;
 
 
       // transition_color(
       //      &mut pwm_blue_green,
       //      &mut pwm_red,
       //      COLOR2,
       //      COLOR1,
       //      Duration::from_secs(1),
       //  )
       //  .await;
    }
}
 
fn set_color(
    pwm_blue_green: &mut Pwm<'_>,
    pwm_red: &mut Pwm<'_>,
    color: (u8, u8, u8),
) {
    let (red, green, blue) = color;
    let mut my_config1 = PwmConfig::default();
    my_config1.top = 255;
    my_config1.compare_a = 255 - blue as u16;
    my_config1.compare_b = 255 - green as u16;
 
    let mut my_config2 = PwmConfig::default();
    my_config2.top = 255;
    my_config2.compare_a = 255 - red as u16;
 
    pwm_blue_green.set_config(&my_config1);
    pwm_red.set_config(&my_config2);
 
}
 
async fn transition_color(
    pwm_blue_green: &mut Pwm<'_>,
    pwm_red: &mut Pwm<'_>,
    start: (u8, u8, u8),
    end: (u8, u8, u8),
    duration: Duration,
) {
    let steps = 100;
    let delay = duration / steps;
 
    for i in 0..=steps {
        let red = interpolate(start.0, end.0, i, steps);
        let green = interpolate(start.1, end.1, i, steps);
        let blue = interpolate(start.2, end.2, i, steps);
 
        let mut my_config1 = PwmConfig::default();
        my_config1.top = 255;
        my_config1.compare_a = 255 - blue as u16;
        my_config1.compare_b = 255 - green as u16;
 
        let mut my_config2 = PwmConfig::default();
        my_config2.top = 255;
        my_config2.compare_a = 255 - red as u16;
 
        pwm_blue_green.set_config(&my_config1);
        pwm_red.set_config(&my_config2);
        Timer::after(delay).await;
    }
}
 
fn interpolate(start: u8, end: u8, step: u32, steps: u32) -> u8 {
    (start as i32  + ((end as i32 - start as i32) as i32 * step as i32  / steps as i32) ) as u8
}
