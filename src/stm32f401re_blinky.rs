//! Minimal blinky example for the STM32F401RE using stm32f4xx-hal.
//!
//! This module provides a simple function to blink the onboard LED (PA5) in a loop.
//! Intended for demonstration and hardware testing purposes.

use stm32f4xx_hal::{pac, prelude::*};

/// Blinks the onboard LED (PA5) in an infinite loop.
///
/// Initializes the GPIOA peripheral, configures PA5 as a push-pull output,
/// and toggles it with a fixed delay. Intended to be called from `main()`.
pub fn blinky() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let _clocks = rcc.cfgr.freeze();
    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();

    loop {
        led.set_high();
        cortex_m::asm::delay(5_000_000);
        led.set_low();
        cortex_m::asm::delay(5_000_000);
    }
}
