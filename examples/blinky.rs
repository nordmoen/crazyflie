//! Simple example to show how to blink with the LEDs
#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m;
use cortex_m_rt::entry;
use crazyflie::hal::{self, prelude::*, stm32};
use crazyflie::led::{Leds, LedN};

#[entry]
fn main() -> ! {
    // Get handles to device peripherals
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();
    // Get references to correct GPIO pins for LEDs
    let gpioc = dp.GPIOC.split();
    let gpiod = dp.GPIOD.split();
    // Initialize LEDs
    let mut leds = Leds::new(gpioc, gpiod);
    // Setup system clock for delay handling
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();
    // Create delay abstraction
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);
    // Loop forever blinking LEDs
    leds.clear_all();
    loop {
        for led_idx in &[LedN::RedLeft, LedN::GreenLeft, LedN::RedRight, LedN::GreenRight, LedN::BlueLeft] {
            leds[*led_idx].on();
            delay.delay_ms(500_u32);
            leds[*led_idx].off();
        }
    }
}
