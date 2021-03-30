//! Simple example to show how to activate the motors
//!
//! Ensure that the propellers are free to spin, or even better remove them before testing!
#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m;
use cortex_m_rt::entry;
use crazyflie::hal::{self, prelude::*, stm32};
use crazyflie::motor::Motors;

#[entry]
fn main() -> ! {
    // Get handles to device peripherals
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();
    // Get references to correct GPIO pins for motors
    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();
    // Setup system clock for delay handling
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();
    // Initialize the motors
    let mut motors = Motors::new(clocks, dp.TIM2, dp.TIM4, gpioa, gpiob);
    // Create delay abstraction
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);
    // Before starting wait a bit so that users can set down the drone
    delay.delay_ms(1000_u32);
    // Make sure to enable the motors
    motors.enable();
    // Start testing motors
    motors.m1.set_duty(150);
    motors.m2.set_duty(150);
    motors.m3.set_duty(150);
    motors.m4.set_duty(150);
    delay.delay_ms(5000_u32);
    // Once done, ensure motors are off
    motors.stop();
    motors.disable();
    // Loop forever after done testing motors
    loop {
    }
}

