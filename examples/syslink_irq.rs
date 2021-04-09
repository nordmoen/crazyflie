//! An example to show how to read `Syslink` messages from the `nRF51`
#![no_main]
#![no_std]
use panic_halt as _;

use cortex_m;
use cortex_m::interrupt::Mutex;
use core::cell::RefCell;
use cortex_m_rt::entry;
use crazyflie::hal::{self, prelude::*, stm32, serial, nb};
use crazyflie::led::{LedN, Leds};
use crazyflie::hal::stm32::interrupt;
use crazyflie::uart_syslink::{UartComm};


static LEDS: Mutex<RefCell<Option<Leds>>> = Mutex::new(RefCell::new(None));
static SYSLINK: Mutex<RefCell<Option<UartComm>>> = Mutex::new(RefCell::new(None));

#[interrupt]
fn USART6() {
    let mut led = None;
    // Try to receive data on Syslink
    cortex_m::interrupt::free(|cs| {
        if let Some(ref mut comm) = *SYSLINK.borrow(cs).borrow_mut() {
            match comm.receive() {
                Ok(_) => led = Some(LedN::GreenLeft),
                Err(nb::Error::WouldBlock) => led = Some(LedN::BlueLeft),
                Err(nb::Error::Other(_)) => led = Some(LedN::RedRight),
            }
        } else {
            led = Some(LedN::RedLeft);
        }
    });
    // If we set a LED we turn it on here, this will create a blinking effect since all LEDs will
    // be turned off in the main loop
    if let Some(ledn) = led {
        cortex_m::interrupt::free(|cs| {
            if let Some(ref mut leds) = *LEDS.borrow(cs).borrow_mut() {
                leds[ledn].on();
            }
        });
    }
}

#[entry]
fn main() -> ! {
    // Get handles to device peripherals
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();
    // Get references to correct GPIO pins for LEDs
    let gpioc = dp.GPIOC.split();
    let gpiod = dp.GPIOD.split();
    // Initialize LEDs
    let mut leds = Leds::new(gpioc.pc0, gpioc.pc1, gpioc.pc2, gpioc.pc3, gpiod.pd2);
    leds.clear_all();
    // Move LEDs into a global so that we can use it in the interrupt
    cortex_m::interrupt::free(|cs| *LEDS.borrow(cs).borrow_mut() = Some(leds));
    // Setup system clock for delay handling
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();
    // Create delay abstraction
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);
    // Create Syslink interface over UART
    let mut comm = UartComm::new(dp.USART6, gpioc.pc6, gpioc.pc7, clocks);
    // Enable interrupt for receiving before moving into global scope
    comm.enable_interrupt(serial::Event::Rxne);
    cortex_m::interrupt::free(|cs| *SYSLINK.borrow(cs).borrow_mut() = Some(comm));
    // Now we must tell the hardware that we want to receive interrupts
    unsafe {cortex_m::peripheral::NVIC::unmask(stm32::Interrupt::USART6)};
    // Loop forever turning off LEDs so that they "blink" if interrupts happen
    loop {
        cortex_m::asm::wfi();
        delay.delay_ms(300u32);
        cortex_m::interrupt::free(|cs| {
            if let Some(ref mut leds) = *LEDS.borrow(cs).borrow_mut() {
                leds.clear_all();
            }
        });
    }
}
