#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
mod stm32f401re_blinky;

#[entry]
fn main() -> ! {
    // stm32f401re_blinky::blinky(); // Uncomment for a minimal blinky example on STM32F401RE platform
    loop {}
}
