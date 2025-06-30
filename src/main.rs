#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    // Acess peripherals
    let dp = pac::Peripherals::take().unwrap();

    // Enable RCC and configure clocks
    let rcc = dp.RCC.constrain();
    let _clocks = rcc.cfgr.freeze();

    // Split GPIOA
    let gpioa = dp.GPIOA.split();

    // Conf. PA5 as output
    let mut led = gpioa.pa5.into_push_pull_output();

    loop {
        led.set_high();
        cortex_m::asm::delay(5_000_000);
        led.set_low();
        cortex_m::asm::delay(5_000_000);
    }
}
