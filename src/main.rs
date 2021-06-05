#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m;
use cortex_m_rt::entry;
use stm32f4xx_hal;

use crate::stm32f4xx_hal::{prelude::*, stm32};

#[entry]
fn main() -> ! {

    loop {}

}