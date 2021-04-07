//! Access to internal persistent storage
//!
//! Currently this module simply exposes the underlying [`eeprom24x::Eeprom24x`] instance by
//! helping to build the correct setup.
use eeprom24x::{Eeprom24x, SlaveAddr, addr_size::TwoBytes, page_size::B32};
use crate::hal::gpio::{AlternateOD, AF4, gpiob::{self, PB6, PB7}};
use crate::hal::i2c;
use crate::hal::pac::I2C1;
use crate::hal::rcc::Clocks;
use crate::hal::prelude::*;

/// I2C instance which is used to communicate with on-board EEPROM
pub type I2c = i2c::I2c<I2C1, (PB6<AlternateOD<AF4>>, PB7<AlternateOD<AF4>>)>;
/// Full type of on-board EEPROM
pub type Eeprom = Eeprom24x<I2c, B32, TwoBytes>;

/// I2C speed in kHz
const I2C_SPEED_KHZ: u32 = 400;

/// Create a connection to the on-board EEPROM
pub fn new(i2c1: I2C1, gpio: gpiob::Parts, clocks: Clocks) -> Eeprom {
    let scl = gpio.pb6.into_alternate_af4_open_drain();
    let sda = gpio.pb7.into_alternate_af4_open_drain();
    let i2c: I2c = i2c::I2c::new(i2c1, (scl, sda), I2C_SPEED_KHZ.khz(), clocks);
    Eeprom24x::new_24x64(i2c, SlaveAddr::default())
}
