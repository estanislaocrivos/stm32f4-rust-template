#![no_std] // On embedded targets we do not include the std. library
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _; // Tells the compiler how to handle panic!
use stm32f4xx_hal::{pac, prelude::*};

#[entry] // Indicates where main() starts.
fn main() -> ! {
    // Minimal blinky example

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
