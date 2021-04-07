//! Board support crate for the main processor (`STM32F405`) of the [Crazyflie
//! 2.1](https://www.bitcraze.io)
#![no_std]

pub use stm32f4xx_hal as hal;

#[cfg(feature = "eeprom")]
pub mod eeprom;
pub mod led;
pub mod motor;
