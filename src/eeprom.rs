//! Access to internal persistent storage
//!
//! Currently this module simply exposes the underlying [`eeprom24x::Eeprom24x`] instance by
//! helping to build the correct setup.
use crate::hal::gpio::{
    gpiob::{PB6, PB7},
    AlternateOD, Floating, Input, AF4,
};
use crate::hal::i2c;
use crate::hal::pac::I2C1;
use crate::hal::prelude::*;
use crate::hal::rcc::Clocks;
use eeprom24x::{addr_size::TwoBytes, page_size::B32, Eeprom24x, SlaveAddr};

/// I2C instance which is used to communicate with on-board EEPROM
pub type I2c = i2c::I2c<I2C1, (PB6<AlternateOD<AF4>>, PB7<AlternateOD<AF4>>)>;
/// Full type of on-board EEPROM
pub type Eeprom = Eeprom24x<I2c, B32, TwoBytes>;

/// I2C speed in kHz
const I2C_SPEED_KHZ: u32 = 400;

/// Create a connection to the on-board EEPROM
pub fn new(
    i2c1: I2C1,
    scl_pin: PB6<Input<Floating>>,
    sda_pin: PB7<Input<Floating>>,
    clocks: Clocks,
) -> Eeprom {
    let scl = scl_pin.into_alternate_af4_open_drain();
    let sda = sda_pin.into_alternate_af4_open_drain();
    let i2c: I2c = i2c::I2c::new(i2c1, (scl, sda), I2C_SPEED_KHZ.khz(), clocks);
    Eeprom24x::new_24x64(i2c, SlaveAddr::default())
}
