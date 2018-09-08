#![no_main]
#![no_std]

#[macro_use]
extern crate cortex_m_rt;
extern crate nb;

extern crate nordic_nrf52840dk;
extern crate panic_semihosting;
extern crate cortex_m;

use nordic_nrf52840dk::hal::{prelude::*, gpio::*, delay::*};
use nordic_nrf52840dk::nrf52840::{Peripherals};
use nordic_nrf52840dk::Pins;

pub type Led1 = p0::P0_13<Output<PushPull>>;
pub type Led2 = p0::P0_14<Output<PushPull>>;
pub type Led3 = p0::P0_15<Output<PushPull>>;
pub type Led4 = p0::P0_16<Output<PushPull>>;

pub struct Leds {
    leds: [Led; 4],
}


entry!(main);

fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let cperi = cortex_m::Peripherals::take().unwrap();
    let pins = Pins::new(p.P0.split());

    let led1 = pins.led1.into_push_pull_output();
    let led2 = pins.led2.into_push_pull_output();
    let led3 = pins.led3.into_push_pull_output();
    let led4 = pins.led4.into_push_pull_output();

    let mut state = false;
    let mut leds = Leds {
        leds:
        [
            led1.into(),
            led2.into(),
            led3.into(),
            led4.into()
        ],
    };

    let clock = p.CLOCK.constrain();
    let clocks = clock.freeze();
    let sys = cperi.SYST;
    let mut del = Delay::new(sys, clocks);



    loop {
        for led in leds.leds.iter_mut() {
            if state
                {
                    led.off();
                } else { led.on(); }
            del.delay_ms(25u32);
        }
        state = !state;
    }
}

/// One of the on-board user LEDs
pub struct Led {
    pex: p0::P0_Pin<Output<PushPull>>,
}

macro_rules! ctor {
    ($($ldx:ident),+) => {
        $(
            impl Into<Led> for $ldx {
                fn into(self) -> Led {
                    Led {
                        pex: self.degrade(),
                    }
                }
            }
        )+
    }
}

ctor!(Led1, Led2, Led3, Led4);

impl Led {
    /// Turns the LED off
    pub fn off(&mut self) {
        self.pex.set_high();
    }

    /// Turns the LED on
    pub fn on(&mut self) {
        self.pex.set_low();
    }
}