//! LED control on the Crazyflie
//!
//! # Usage
//! Instantiate the [`Leds`](Leds::new) structure and use [`LedN`] to index this structure to
//! access the individual LEDs. The interface to each LED is controlled through [`Led`].

use crate::hal::gpio::gpioc::{self, PC, PC0, PC1, PC2, PC3};
use crate::hal::gpio::gpiod::{self, PD, PD2};
use crate::hal::gpio::{Output, PushPull, Speed};
use crate::hal::prelude::*;
use core::ops::{Index, IndexMut};

/// Blue LED on the left
pub type BlueLEDLeft = PD2<Output<PushPull>>;
/// Red LED on the left
pub type RedLEDLeft = PC0<Output<PushPull>>;
/// Green LED on the left
pub type GreenLEDLeft = PC1<Output<PushPull>>;
/// Green LED on the right
pub type GreenLEDRight = PC2<Output<PushPull>>;
/// Red LED on the right
pub type RedLEDRight = PC3<Output<PushPull>>;

/// Abstraction over one of the LEDs on the Crazyflie
pub enum Led {
    /// LED on GPIOC
    ///
    /// These LEDs have reverse polarity compared to [`Led::LedD`]
    LedC(PC<Output<PushPull>>),
    /// LED on GPIOD
    LedD(PD<Output<PushPull>>),
}

impl Led {
    /// Turn LED off
    pub fn off(&mut self) {
        match self {
            // The following unwraps can never fail since the error is `Infallible`
            Led::LedC(pin) => pin.set_high().unwrap(),
            Led::LedD(pin) => pin.set_low().unwrap(),
        }
    }

    /// Turn the LED on
    pub fn on(&mut self) {
        match self {
            // The following unwraps can never fail since the error is `Infallible`
            Led::LedC(pin) => pin.set_low().unwrap(),
            Led::LedD(pin) => pin.set_high().unwrap(),
        }
    }

    /// Check if the LED is turned on
    pub fn is_on(&self) -> bool {
        match self {
            // The following unwraps can never fail since the error is `Infallible`
            Led::LedC(pin) => pin.is_set_low().unwrap(),
            Led::LedD(pin) => pin.is_set_high().unwrap(),
        }
    }

    /// Check if the LED is turned off
    pub fn is_off(&self) -> bool {
        !self.is_on()
    }
}

/// A specific LED, use this to index [`Leds`] to get desired LED
#[derive(Copy, Clone, Debug)]
pub enum LedN {
    /// Red LED on the left
    RedLeft,
    /// Green LED on the left
    GreenLeft,
    /// Blue LED on the left
    BlueLeft,
    /// Red LED on the right
    RedRight,
    /// Greed LED on the right
    GreenRight,
}

/// Container for LEDs on the Crazyflie
pub struct Leds {
    leds: [Led; 5],
}

impl Leds {
    /// Initialize the LEDs on the Crazyflie
    pub fn new(gpioc: gpioc::Parts, gpiod: gpiod::Parts) -> Self {
        let red_left = Led::LedC(
            gpioc
                .pc0
                .into_push_pull_output()
                .set_speed(Speed::Medium)
                .downgrade(),
        );
        let green_left = Led::LedC(
            gpioc
                .pc1
                .into_push_pull_output()
                .set_speed(Speed::Medium)
                .downgrade(),
        );
        let blue_left = Led::LedD(
            gpiod
                .pd2
                .into_push_pull_output()
                .set_speed(Speed::Medium)
                .downgrade(),
        );
        let green_right = Led::LedC(
            gpioc
                .pc2
                .into_push_pull_output()
                .set_speed(Speed::Medium)
                .downgrade(),
        );
        let red_right = Led::LedC(
            gpioc
                .pc3
                .into_push_pull_output()
                .set_speed(Speed::Medium)
                .downgrade(),
        );
        Leds {
            leds: [red_left, green_left, blue_left, red_right, green_right],
        }
    }

    /// Turn off all LEDs
    pub fn clear_all(&mut self) {
        self.leds.iter_mut().for_each(|l| l.off());
    }

    /// Turn all LEDs on
    pub fn set_all(&mut self) {
        self.leds.iter_mut().for_each(|l| l.on());
    }
}

impl Index<LedN> for Leds {
    type Output = Led;

    fn index(&self, led: LedN) -> &Self::Output {
        &self.leds[led as usize]
    }
}

impl IndexMut<LedN> for Leds {
    fn index_mut(&mut self, led: LedN) -> &mut Self::Output {
        &mut self.leds[led as usize]
    }
}
