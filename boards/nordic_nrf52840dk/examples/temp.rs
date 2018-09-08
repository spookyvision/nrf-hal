#![no_main]
#![no_std]

#[macro_use]
extern crate cortex_m_rt;
extern crate nb;

extern crate nordic_nrf52840dk;
extern crate panic_semihosting;
extern crate cortex_m_semihosting as sh;
extern crate cortex_m;

use nordic_nrf52840dk::hal::{prelude::*, delay::*};
use core::fmt::Write;

entry!(main);

fn main() -> ! {

    let peripherals = nordic_nrf52840dk::nrf52840::Peripherals::take().unwrap();
    let cortex_periphs = cortex_m::Peripherals::take().unwrap();
    let clock = peripherals.CLOCK.constrain();
    let clocks = clock.freeze();
    let sys = cortex_periphs.SYST;
    let mut del = Delay::new(sys, clocks);
    let temp_struct = peripherals.TEMP;

    loop {
        let temp = nordic_nrf52840dk::temperature::read_temperature(&temp_struct);
        del.delay_ms(1000u32);
        write!(sh::hio::hstdout().unwrap(), "Temp is {} \n", temp);
    }
}