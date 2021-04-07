//! Example to read the Magic number of the Crazyflie Config block
#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m;
use cortex_m_rt::entry;
use crazyflie::eeprom;
use crazyflie::hal::{self, prelude::*, stm32};
use crazyflie::led::{LedN, Leds};

const MAGIC_BLOCK_NUMBER: u32 = 0x43427830;

#[entry]
fn main() -> ! {
    // Get handles to device peripherals
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();
    // Get references to correct GPIO pins for LEDs
    let gpiob = dp.GPIOB.split();
    let gpioc = dp.GPIOC.split();
    let gpiod = dp.GPIOD.split();
    // Initialize LEDs
    let mut leds = Leds::new(gpioc.pc0, gpioc.pc1, gpioc.pc2, gpioc.pc3, gpiod.pd2);
    // Setup system clock for delay handling
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();
    // Create delay abstraction
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);
    // Create EEPROM connection
    let mut eeprom = eeprom::new(dp.I2C1, gpiob.pb6, gpiob.pb7, clocks);
    // Clear LEDs so that we can use it to signal success
    leds.clear_all();
    // Try to read magic number from EEPROM
    let mut magic_bytes = [0; 4];
    if let Ok(_) = eeprom.read_data(0u32, &mut magic_bytes) {
        let magic = u32::from_ne_bytes(magic_bytes);
        if magic == MAGIC_BLOCK_NUMBER {
            loop {
                leds[LedN::GreenRight].on();
                delay.delay_ms(300u32);
                leds[LedN::GreenRight].off();
                delay.delay_ms(300u32);
            }
        } else {
            loop {
                leds[LedN::RedRight].on();
                delay.delay_ms(300u32);
                leds[LedN::RedRight].off();
                delay.delay_ms(300u32);
            }
        }
    } else {
        // Read error signal with left red LED
        loop {
            leds[LedN::RedLeft].on();
            delay.delay_ms(300u32);
            leds[LedN::RedLeft].off();
            delay.delay_ms(300u32);
        }
    }
}
