#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
mod stm32f401re_blinky;

#[entry] // Indicates where main() starts.
fn main() -> ! {
    // Uncomment next line for a minimal blinky example on STM32F401RE platform
    stm32f401re_blinky::blinky();
    // loop {}
}
