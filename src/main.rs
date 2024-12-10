#![no_main]
#![no_std]

use panic_semihosting as _;

use cortex_m_semihosting::hprint;
use stm32f1xx_hal as _;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    hprint!("BEGIN PROGRAM!!!");
    loop {
        hprint!("Hello, world!");
        cortex_m::asm::delay(10_000_000);
    }
}