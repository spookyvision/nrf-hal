#![no_std]
pub extern crate nrf52840_hal as hal;
pub extern crate cortex_m_rt as rt;
pub extern crate nb;
extern crate panic_semihosting;
extern crate cortex_m;

use hal::gpio::{p0, Floating, Input};
pub use hal::nrf52840;

pub mod temperature;

/// Maps the pins to the names printed on the device
pub struct Pins {
    pub xl1     :p0::P0_0 <Input<Floating>>,
    pub xl2     :p0::P0_1 <Input<Floating>>,
    pub button1 :p0::P0_11<Input<Floating>>,
    pub button2 :p0::P0_12<Input<Floating>>,
    pub button3 :p0::P0_24<Input<Floating>>,
    pub button4 :p0::P0_25<Input<Floating>>,
    pub led1    :p0::P0_13<Input<Floating>>,
    pub led2    :p0::P0_14<Input<Floating>>,
    pub led3    :p0::P0_15<Input<Floating>>,
    pub led4    :p0::P0_16<Input<Floating>>,
    pub cs      :p0::P0_17<Input<Floating>>,
    pub clk     :p0::P0_19<Input<Floating>>,
    pub dio0    :p0::P0_20<Input<Floating>>,
    pub dio1    :p0::P0_21<Input<Floating>>,
    pub dio2    :p0::P0_22<Input<Floating>>,
    pub dio3    :p0::P0_23<Input<Floating>>,

}

impl Pins {
    pub fn new(pins: p0::Parts) -> Self {
        Self {
            xl1     :  pins.p0_0 ,
            xl2     :  pins.p0_1 ,
            button1 :  pins.p0_11,
            button2 :  pins.p0_12,
            button3 :  pins.p0_24,
            button4 :  pins.p0_25,
            led1    :  pins.p0_13,
            led2    :  pins.p0_14,
            led3    :  pins.p0_15,
            led4    :  pins.p0_16,
            cs      :  pins.p0_17,
            clk     :  pins.p0_19,
            dio0    :  pins.p0_20,
            dio1    :  pins.p0_21,
            dio2    :  pins.p0_22,
            dio3    :  pins.p0_23,
        }
    }
}

