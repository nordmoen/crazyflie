//! Interface to the Motors of the Crazyflie
//!
//! # Usage
//! Instantiate the [`Motors`](Motors::new) structure to get access to individual [`Motor`]s.

use crate::hal::{
    gpio::{gpioa, gpiob},
    prelude::*,
    pwm,
    rcc::Clocks,
    stm32::{TIM2, TIM4},
};

/// PWM clock rate in Hz
///
/// The constant value comes from official CF2 firmware which report better filter ripple at 328kHz
/// https://github.com/bitcraze/crazyflie-firmware/blob/master/src/drivers/interface/motors.h#L46
const CLOCK_KHZ: u32 = 328;

/// Connection type of Motor 1
pub type Motor1 = pwm::PwmChannels<TIM2, pwm::C2>;
/// Connection type of Motor 2
pub type Motor2 = pwm::PwmChannels<TIM2, pwm::C4>;
/// Connection type of Motor 3
pub type Motor3 = pwm::PwmChannels<TIM2, pwm::C1>;
/// Connection type of Motor 4
pub type Motor4 = pwm::PwmChannels<TIM4, pwm::C4>;

/// Abstraction around a single motor
pub enum Motor {
    M1(Motor1),
    M2(Motor2),
    M3(Motor3),
    M4(Motor4),
}

impl Motor {
    /// Enable the underlying `PWM` channel
    pub fn enable(&mut self) {
        match self {
            Motor::M1(m) => m.enable(),
            Motor::M2(m) => m.enable(),
            Motor::M3(m) => m.enable(),
            Motor::M4(m) => m.enable(),
        }
    }

    /// Disable the underlying `PWM` channel
    pub fn disable(&mut self) {
        match self {
            Motor::M1(m) => m.disable(),
            Motor::M2(m) => m.disable(),
            Motor::M3(m) => m.disable(),
            Motor::M4(m) => m.disable(),
        }
    }

    /// Returns the underlying `PWM` duty cycle
    pub fn get_duty(&self) -> u16 {
        match self {
            Motor::M1(m) => m.get_duty(),
            Motor::M2(m) => m.get_duty(),
            Motor::M3(m) => m.get_duty(),
            Motor::M4(m) => m.get_duty(),
        }
    }

    /// Get the maximum duty value for the underlying `PWM` pin
    pub fn get_max_duty(&self) -> u16 {
        match self {
            Motor::M1(m) => m.get_max_duty(),
            Motor::M2(m) => m.get_max_duty(),
            Motor::M3(m) => m.get_max_duty(),
            Motor::M4(m) => m.get_max_duty(),
        }
    }

    /// Set the duty cycle for the underlying `PWM`
    pub fn set_duty(&mut self, duty: u16) {
        match self {
            Motor::M1(m) => m.set_duty(duty),
            Motor::M2(m) => m.set_duty(duty),
            Motor::M3(m) => m.set_duty(duty),
            Motor::M4(m) => m.set_duty(duty),
        }
    }

    /// Set the motor thrust as a ratio of `thrust / 65536`
    ///
    /// # Arguments
    /// - `thrust` - ratio of thrust
    /// - `voltage` - battery voltage
    pub fn set_ratio(&mut self, thrust: u16, voltage: f32) {
        let thrust = (thrust as f32 / 65536.0) * 60.0;
        // https://github.com/bitcraze/crazyflie-firmware/blob/master/src/drivers/src/motors.c#L237
        let volts = -0.0006239 * thrust * thrust + 0.088 * thrust;
        let percent = (volts / voltage).clamp(0.0, 1.0);
        self.set_duty((percent * u16::MAX as f32) as u16);
    }
}

/// Container for all motors
pub struct Motors {
    pub m1: Motor,
    pub m2: Motor,
    pub m3: Motor,
    pub m4: Motor,
}

impl Motors {
    /// Initialize the motors
    pub fn new(
        clocks: Clocks,
        tim2: TIM2,
        tim4: TIM4,
        gpioa: gpioa::Parts,
        gpiob: gpiob::Parts,
    ) -> Self {
        // Create motors connected to TIM2
        let tim2_pins = (
            gpioa.pa15.into_alternate_af1(),
            gpioa.pa1.into_alternate_af1(),
            gpiob.pb11.into_alternate_af1(),
        );
        let (m3, m1, m2) = pwm::tim2(tim2, tim2_pins, clocks, CLOCK_KHZ.khz());
        // Create motors connected to TIM4
        let tim4_pin = gpiob.pb9.into_alternate_af2();
        let m4 = pwm::tim4(tim4, tim4_pin, clocks, CLOCK_KHZ.khz());
        // Return configured motors
        let mut ms = Motors {
            m1: Motor::M1(m1),
            m2: Motor::M2(m2),
            m3: Motor::M3(m3),
            m4: Motor::M4(m4),
        };
        // Ensure that the motors are stopped when we give away control
        ms.stop();
        ms
    }

    /// Enable all motors
    pub fn enable(&mut self) {
        self.m1.enable();
        self.m2.enable();
        self.m3.enable();
        self.m4.enable();
    }

    /// Disable all motors
    pub fn disable(&mut self) {
        self.m1.disable();
        self.m2.disable();
        self.m3.disable();
        self.m4.disable();
    }

    /// Stop all motors
    pub fn stop(&mut self) {
        self.m1.set_duty(0);
        self.m2.set_duty(0);
        self.m3.set_duty(0);
        self.m4.set_duty(0);
    }
}
