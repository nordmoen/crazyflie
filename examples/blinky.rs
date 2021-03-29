//! Simple example to show how to blink with the LEDs
#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m;
use cortex_m_rt::entry;
use crazyflie::hal::{self, prelude::*, stm32};

#[entry]
fn main() -> ! {
    // Get handles to device peripherals
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();
    // Get references to correct GPIO pins for LEDs
    let gpioc = dp.GPIOC.split();
    let gpiod = dp.GPIOD.split();
    // Initialize LEDs
    let mut leds = crazyflie::led::Leds::new(gpioc, gpiod);
    // Setup system clock for delay handling
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();
    // Create delay abstraction
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);
    // Loop forever blinking LEDs
    loop {
        leds.set_all();
        delay.delay_ms(500_u32);
        leds.clear_all();
        delay.delay_ms(500_u32);
    }
}
