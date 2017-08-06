//! User LEDs
//!
//! - Orange = PC13

use stm32f41x::{GPIOA, RCC};

/// Orange LED (PA8)
pub struct Orange;

/// Initializes the user LED
pub fn init(gpioa: &GPIOA, rcc: &RCC) {
    // power on GPIOA
    rcc.ahb1enr.modify(|_, w| w.gpioaen().set_bit());

    // configure PC13 as output
    gpioa.bsrr.write(|w| w.bs8().set_bit());
    gpioa.moder.modify(|_, w| unsafe { w.moder8().bits(1) } );
}

impl Orange {
    /// Turns the LED on
    pub fn on(&self) {
        unsafe {
            (*GPIOA.get()).bsrr.write(|w| w.br8().set_bit());
        }
    }

    /// Turns the LED off
    pub fn off(&self) {
        unsafe {
            (*GPIOA.get()).bsrr.write(|w| w.bs8().set_bit());
        }
    }
}
