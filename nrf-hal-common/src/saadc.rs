//! HAL interface to the SAADC peripheral.
//!
//! Example usage:
//!
#![cfg_attr(feature = "52840", doc = "```no_run")]
#![cfg_attr(not(feature = "52840"), doc = "```ignore")]
//! # use nrf_hal_common as hal;
//! # use hal::pac::{saadc, SAADC};
//! // subsititute `hal` with the HAL of your board, e.g. `nrf52840_hal`
//! use hal::{
//!    pac::Peripherals,
//!    prelude::*,
//!    gpio::p0::Parts as P0Parts,
//!    saadc::{SaadcConfig, Saadc},
//! };
//!
//! let board = Peripherals::take().unwrap();
//! let gpios = P0Parts::new(board.P0);
//!
//! // initialize saadc interface
//! let saadc_config = SaadcConfig::default();
//! let mut saadc = Saadc::new(board.SAADC, saadc_config);
//! let mut saadc_pin = gpios.p0_02; // the pin your analog device is connected to
//!
//! // blocking read from saadc for `saadc_config.time` microseconds
//! let _saadc_result = saadc.read(&mut saadc_pin);
//! ```

#[cfg(feature = "9160")]
use crate::pac::{saadc_ns as saadc, SAADC_NS as SAADC};

#[cfg(not(feature = "9160"))]
use crate::pac::{saadc, SAADC};

use crate::gpio::{Floating, Input, Pin};
use crate::target_constants::{EASY_DMA_SIZE, FORCE_COPY_BUFFER_SIZE};
use crate::{slice_in_ram, slice_in_ram_or, DmaSlice};

use core::marker::PhantomData;
use core::{
    hint::unreachable_unchecked,
    sync::atomic::{compiler_fence, Ordering::SeqCst},
};
use embedded_hal::adc::{Channel, OneShot};

use embedded_dma::{ReadBuffer, WriteBuffer};

pub use saadc::{
    ch::config::{GAIN_A as Gain, REFSEL_A as Reference, RESP_A as Resistor, TACQ_A as Time},
    oversample::OVERSAMPLE_A as Oversample,
    resolution::VAL_A as Resolution,
};

// Only 1 channel is allowed right now, a discussion needs to be had as to how
// multiple channels should work (See "scan mode" in the datasheet).
// Issue: https://github.com/nrf-rs/nrf-hal/issues/82

/// Interface for the SAADC peripheral.
///
/// External analog channels supported by the SAADC implement the `Channel` trait.
/// Currently, use of only one channel is allowed.
pub struct Saadc1(SAADC);

pub type AdcPin = Option<Pin<Input<Floating>>>;

// TODO copypasta from `spim.rs` except TxBufferTooLong
#[derive(Debug)]
pub enum Error {
    RxBufferTooLong,
    /// EasyDMA can only read from data memory, read only buffers in flash will fail.
    DMABufferNotInDataMemory,
    Transmit,
    Receive,
    NextTransferAlreadyEnqueued,
    CurrentTransferStillPending,
}

pub struct Saadc {
    periph: SAADC,
    pin: AdcPin,
}

pub struct Transfer<RxB>
where
    RxB: WriteBuffer,
{
    inner: Option<InnerTransfer<RxB>>,
}

pub struct InnerTransfer<RxB>
where
    RxB: WriteBuffer,
{
    rx_buffer: RxB,
    saadc: Saadc,
    next_queued: bool,
}

pub struct PendingTransfer<RxB>
where
    RxB: WriteBuffer,
{
    rx_buffer: RxB,
    _phantom: PhantomData<Saadc>,
}

// TODO copypasta from `spim.rs`
#[inline(always)]
fn wb_to_dma_slice<WB: WriteBuffer>(wb: &mut WB) -> DmaSlice {
    let (ptr, len) = unsafe { wb.write_buffer() };
    DmaSlice {
        ptr: ptr as usize as u32,
        len: (len * core::mem::size_of::<WB::Word>()) as u32,
    }
}

impl Saadc {
    pub fn new(saadc: SAADC, pin: AdcPin, config: SaadcConfig) -> Self {
        // The write enums do not implement clone/copy/debug, only the
        // read ones, hence the need to pull out and move the values.
        let SaadcConfig {
            resolution,
            oversample,
            reference,
            gain,
            resistor,
            time,
        } = config;

        saadc.enable.write(|w| w.enable().enabled());
        saadc.resolution.write(|w| w.val().variant(resolution));
        saadc
            .oversample
            .write(|w| w.oversample().variant(oversample));
        saadc.samplerate.write(|w| w.mode().task());

        saadc.ch[0].config.write(|w| {
            w.refsel().variant(reference);
            w.gain().variant(gain);
            w.tacq().variant(time);
            w.mode().se();
            w.resp().variant(resistor);
            w.resn().bypass();
            w.burst().enabled();
            w
        });
        saadc.ch[0].pseln.write(|w| w.pseln().nc());

        // Calibrate
        saadc.tasks_calibrateoffset.write(|w| unsafe { w.bits(1) });
        while saadc.events_calibratedone.read().bits() == 0 {}

        Saadc { periph: saadc, pin }
    }

    pub fn dma_transfer<RxW, RxB>(
        mut self,
        mut rx_buffer: RxB,
    ) -> Result<Transfer<RxB>, (Self, Error)>
    where
        RxB: WriteBuffer<Word = RxW>,
    {
        let rx_dma = wb_to_dma_slice(&mut rx_buffer);

        // TODO correct check?
        if rx_dma.len as usize > EASY_DMA_SIZE {
            return Err((self, Error::RxBufferTooLong));
        }

        self.start_adc_dma_transfer(&rx_dma);

        Ok(Transfer {
            inner: Some(InnerTransfer {
                rx_buffer,
                saadc: self,
                next_queued: false,
            }),
        })
    }

    fn start_adc_dma_transfer(&mut self, rx: &DmaSlice) {
        // Conservative compiler fence to prevent optimizations that do not
        // take in to account actions by DMA. The fence has been placed here,
        // before any DMA action has started.
        compiler_fence(SeqCst);

        self.set_dma_values(rx);

        self.periph.events_end.write(|w| w.events_end().clear_bit());

        // Start SPI transaction.
        self.periph.tasks_start.write(|w|
            // `1` is a valid value to write to task registers.
            unsafe { w.bits(1) });

        // Conservative compiler fence to prevent optimizations that do not
        // take in to account actions by DMA. The fence has been placed here,
        // after all possible DMA actions have completed.
        compiler_fence(SeqCst);
    }

    fn complete_adc_dma_transfer(&mut self, rx: &DmaSlice) -> Result<usize, Error> {
        // Reset the event, otherwise it will always read `1` from now on.
        self.periph.events_end.write(|w| w.events_end().clear_bit());

        // Conservative compiler fence to prevent optimizations that do not
        // take in to account actions by DMA. The fence has been placed here,
        // after all possible DMA actions have completed.
        compiler_fence(SeqCst);

        let rx_amt = self.periph.result.amount.read().bits();

        if rx_amt != rx.len {
            return Err(Error::Receive);
        }

        Ok(rx_amt as usize)
    }

    fn set_dma_values(&mut self, rx: &DmaSlice) {
        // Conservative compiler fence to prevent optimizations that do not
        // take in to account actions by DMA. The fence has been placed here,
        // before any DMA action has started.
        compiler_fence(SeqCst);

        // Set up the DMA read.
        self.periph.result.ptr.write(|w|
            // This is safe for the same reasons that writing to TXD.PTR is
            // safe. Please refer to the explanation there.
            unsafe { w.ptr().bits(rx.ptr) });
        self.periph.result.maxcnt.write(|w|
            // This is safe for the same reasons that writing to TXD.MAXCNT is
            // safe. Please refer to the explanation there.
            unsafe { w.maxcnt().bits(rx.len as _) });

        compiler_fence(SeqCst);
    }

    #[inline(always)]
    fn is_adc_dma_transfer_complete(&mut self) -> bool {
        self.periph.events_end.read().bits() != 0
    }
}

impl<RxB> Transfer<RxB>
where
    RxB: WriteBuffer,
{
    /// Blocks until the transfer is done and returns the buffer.
    pub fn wait(mut self) -> (RxB, Saadc) {
        compiler_fence(SeqCst);

        let mut inner = self.inner.take().unwrap();

        while !inner.saadc.is_adc_dma_transfer_complete() {}

        // tx, rx
        inner
            .saadc
            .complete_adc_dma_transfer(&wb_to_dma_slice(&mut inner.rx_buffer))
            .ok();

        (inner.rx_buffer, inner.saadc)
    }

    pub fn exchange_transfer_wait(self, pending: PendingTransfer<RxB>) -> (RxB, Self) {
        // TODO: See notes above about validating shortcut, started events, etc.

        let (old_rxb, saadc) = self.wait();
        let new = Transfer {
            inner: Some(InnerTransfer {
                rx_buffer: pending.rx_buffer,
                saadc,
                next_queued: false,
            }),
        };

        (old_rxb, new)
    }

    // TODO: This doesn't HAVE to be RxB, we could have a different
    // type (but with the same bounds), but that should be a non-breaking
    // change later
    //
    // TODO: We probably *should* check that the "STARTED" event has happened,
    // and that the start-to-end shortcut is activated
    pub fn enqueue_next_transfer(
        &mut self,
        mut rx_buffer: RxB,
    ) -> Result<PendingTransfer<RxB>, (RxB, Error)> {
        let rx_dma = wb_to_dma_slice(&mut rx_buffer);

        // TODO correct check?
        if rx_dma.len as usize > EASY_DMA_SIZE {
            return Err((rx_buffer, Error::RxBufferTooLong));
        }

        let mut inner = self.inner.as_mut().unwrap();

        if inner.next_queued {
            return Err((rx_buffer, Error::NextTransferAlreadyEnqueued));
        }

        let started = inner
            .saadc
            .periph
            .events_started
            .read()
            .events_started()
            .bit_is_set();
        if started {
            inner
                .saadc
                .periph
                .events_started
                .write(|w| w.events_started().clear_bit());
        } else {
            return Err((rx_buffer, Error::CurrentTransferStillPending));
        }

        inner.next_queued = true;
        inner.saadc.set_dma_values(&rx_dma);

        Ok(PendingTransfer {
            rx_buffer,
            _phantom: PhantomData,
        })
    }

    // TODO: We should probably add `bail` method like `spis`, but it would
    // require thinking about how to clean up, and potentially re-enable.

    /// Checks if the granted transfer is done.
    #[inline(always)]
    pub fn is_done(&mut self) -> bool {
        let inner = self.inner.as_mut().unwrap();
        inner.saadc.is_adc_dma_transfer_complete()
    }
}

// TODO: Should we also impl drop for PendingSplit? Probably!
impl<RxB> Drop for Transfer<RxB>
where
    RxB: WriteBuffer,
{
    fn drop(&mut self) {
        if let Some(inner) = self.inner.take() {
            compiler_fence(SeqCst);
            inner.saadc.periph.enable.write(|w| w.enable().disabled());
        }
    }
}

/// Used to configure the SAADC peripheral.
///
/// See the documentation of the `Default` impl for suitable default values.
pub struct SaadcConfig {
    /// Output resolution in bits.
    pub resolution: Resolution,
    /// Average 2^`oversample` input samples before transferring the result into memory.
    pub oversample: Oversample,
    /// Reference voltage of the SAADC input.
    pub reference: Reference,
    /// Gain used to control the effective input range of the SAADC.
    pub gain: Gain,
    /// Positive channel resistor control.
    pub resistor: Resistor,
    /// Acquisition time in microseconds.
    pub time: Time,
}

/// Default SAADC configuration. 0 volts reads as 0, VDD volts reads as `u16::MAX`.
/// The returned SaadcConfig is configured with the following values:
///
#[cfg_attr(feature = "52840", doc = "```")]
#[cfg_attr(not(feature = "52840"), doc = "```ignore")]
/// # use nrf_hal_common::saadc::SaadcConfig;
/// # use nrf_hal_common::pac::{saadc, SAADC};
/// # use saadc::{
/// #    ch::config::{GAIN_A as Gain, REFSEL_A as Reference, RESP_A as Resistor, TACQ_A as Time},
/// #    oversample::OVERSAMPLE_A as Oversample,
/// #    resolution::VAL_A as Resolution,
/// # };
/// # let saadc =
/// SaadcConfig {
///     resolution: Resolution::_14BIT,
///     oversample: Oversample::OVER8X,
///     reference: Reference::VDD1_4,
///     gain: Gain::GAIN1_4,
///     resistor: Resistor::BYPASS,
///     time: Time::_20US,
/// };
/// #
/// # // ensure default values haven't changed
/// # let test_saadc = SaadcConfig::default();
/// # assert_eq!(saadc.resolution, test_saadc.resolution);
/// # assert_eq!(saadc.oversample, test_saadc.oversample);
/// # assert_eq!(saadc.reference, test_saadc.reference);
/// # assert_eq!(saadc.gain, test_saadc.gain);
/// # assert_eq!(saadc.resistor, test_saadc.resistor);
/// # assert_eq!(saadc.time, test_saadc.time);
/// # ()
/// ```
impl Default for SaadcConfig {
    fn default() -> Self {
        // Note: do not forget to update the docs above if you change values here
        SaadcConfig {
            resolution: Resolution::_14BIT,
            oversample: Oversample::OVER8X,
            reference: Reference::VDD1_4,
            gain: Gain::GAIN1_4,
            resistor: Resistor::BYPASS,
            time: Time::_20US,
        }
    }
}

impl<PIN> OneShot<Saadc, i16, PIN> for Saadc
where
    PIN: Channel<Saadc, ID = u8>,
{
    type Error = ();

    /// Sample channel `PIN` for the configured ADC acquisition time in differential input mode.
    /// Note that this is a blocking operation.
    fn read(&mut self, _pin: &mut PIN) -> nb::Result<i16, Self::Error> {
        match PIN::channel() {
            0 => self.periph.ch[0].pselp.write(|w| w.pselp().analog_input0()),
            1 => self.periph.ch[0].pselp.write(|w| w.pselp().analog_input1()),
            2 => self.periph.ch[0].pselp.write(|w| w.pselp().analog_input2()),
            3 => self.periph.ch[0].pselp.write(|w| w.pselp().analog_input3()),
            4 => self.periph.ch[0].pselp.write(|w| w.pselp().analog_input4()),
            5 => self.periph.ch[0].pselp.write(|w| w.pselp().analog_input5()),
            6 => self.periph.ch[0].pselp.write(|w| w.pselp().analog_input6()),
            7 => self.periph.ch[0].pselp.write(|w| w.pselp().analog_input7()),
            #[cfg(not(feature = "9160"))]
            8 => self.periph.ch[0].pselp.write(|w| w.pselp().vdd()),
            // This can never happen the only analog pins have already been defined
            // PAY CLOSE ATTENTION TO ANY CHANGES TO THIS IMPL OR THE `channel_mappings!` MACRO
            _ => unsafe { unreachable_unchecked() },
        }

        let mut val: i16 = 0;
        self.periph
            .result
            .ptr
            .write(|w| unsafe { w.ptr().bits(((&mut val) as *mut _) as u32) });
        self.periph
            .result
            .maxcnt
            .write(|w| unsafe { w.maxcnt().bits(1) });

        // Conservative compiler fence to prevent starting the ADC before the
        // pointer and maxcount have been set.
        compiler_fence(SeqCst);

        self.periph.tasks_start.write(|w| unsafe { w.bits(1) });
        self.periph.tasks_sample.write(|w| unsafe { w.bits(1) });

        while self.periph.events_end.read().bits() == 0 {}
        self.periph.events_end.reset();

        // Will only occur if more than one channel has been enabled.
        if self.periph.result.amount.read().bits() != 1 {
            return Err(nb::Error::Other(()));
        }

        // Second fence to prevent optimizations creating issues with the EasyDMA-modified `val`.
        compiler_fence(SeqCst);

        Ok(val)
    }
}

macro_rules! channel_mappings {
    ( $($n:expr => $pin:ident,)*) => {
        $(
            impl<STATE> Channel<Saadc> for crate::gpio::p0::$pin<STATE> {
                type ID = u8;

                fn channel() -> <Self as embedded_hal::adc::Channel<Saadc>>::ID {
                    $n
                }
            }
        )*
    };
}

#[cfg(feature = "9160")]
channel_mappings! {
    0 => P0_13,
    1 => P0_14,
    2 => P0_15,
    3 => P0_16,
    4 => P0_17,
    5 => P0_18,
    6 => P0_19,
    7 => P0_20,
}

#[cfg(not(feature = "9160"))]
channel_mappings! {
    0 => P0_02,
    1 => P0_03,
    2 => P0_04,
    3 => P0_05,
    4 => P0_28,
    5 => P0_29,
    6 => P0_30,
    7 => P0_31,
}

#[cfg(not(feature = "9160"))]
impl Channel<Saadc> for InternalVdd {
    type ID = u8;

    fn channel() -> <Self as embedded_hal::adc::Channel<Saadc>>::ID {
        8
    }
}

#[cfg(not(feature = "9160"))]
/// Channel that doesn't sample a pin, but the internal VDD voltage.
pub struct InternalVdd;
