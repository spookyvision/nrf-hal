//! HAL interface for the PPI peripheral
//!
//! The Programmable Peripheral Interconnect interface allows for an autonomous interoperability
//! between peripherals through their events and tasks. There are fixed PPI channels and fully
//! configurable ones, fixed channels can only connect specific events to specific tasks. For fully
//! configurable channels, it is possible to choose, via software, the event and the task that it
//! will triggered by the event.
//!
//! On nRF52 devices, there is also a fork task endpoint, where the user can configure one more task
//! to be triggered by the same event, even fixed PPI channels have a configurable fork task.

use crate::target::PPI;

mod sealed {
    use super::{TaskAddr, EventAddr};

    pub trait Channel {
        const CH: usize;
    }

    pub trait Task {
        fn task_addr(&self) -> TaskAddr {
            TaskAddr(&self as *const _ as u32)
        }
    }
    pub trait Event {
        fn event_addr(&self) -> EventAddr {
            EventAddr(&self as *const _ as u32)
        }
    }

    pub trait NotFixed {}
}
use sealed::{Channel, NotFixed, Task, Event};

pub struct TaskAddr(pub(crate) u32);
pub struct EventAddr(pub(crate) u32);

/// Trait to represent a Programmable Peripheral Interconnect channel.
pub trait Ppi {
    /// Enables the channel.
    fn enable(&mut self);

    /// Disables the channel.
    fn disable(&mut self);

    #[cfg(not(feature = "51"))]
    /// Sets the fork task that must be triggered when the configured event occurs. The user must
    /// provide the address of the task.
    fn set_fork_task_endpoint<T: Task>(&mut self, task: &T);
}

/// Traits that extends the [Ppi](trait.Ppi.html) trait, marking a channel as fully configurable.
pub trait ConfigurablePpi {
    /// Sets the task that must be triggered when the configured event occurs. The user must provide
    /// the address of the task.
    fn set_task_endpoint<T: Task>(&mut self, task: &T);

    /// Sets the event that will trigger the chosen task(s). The user must provide the address of
    /// the event.
    fn set_event_endpoint<E: Event>(&mut self, event: &E);
}

// All unsafe `ptr` calls only uses registers atomically, and only changes the resources owned by
// the type (guaranteed by the abstraction)
impl<P: Channel> Ppi for P {
    fn enable(&mut self) {
        let regs = unsafe { &*PPI::ptr() };
        regs.chenset.write(|w| unsafe { w.bits(1 << P::CH) });
    }

    fn disable(&mut self) {
        let regs = unsafe { &*PPI::ptr() };
        regs.chenclr.write(|w| unsafe { w.bits(1 << P::CH) });
    }

    #[cfg(not(feature = "51"))]
    fn set_fork_task_endpoint<T: Task>(&mut self, task: &T) {
        let regs = unsafe { &*PPI::ptr() };
        regs.fork[P::CH].tep.write(|w| unsafe { w.bits(task.task_addr().0) });
    }
}

// All unsafe `ptr` calls only uses registers atomically, and only changes the resources owned by
// the type (guaranteed by the abstraction)
impl<P: Channel + NotFixed> ConfigurablePpi for P {
    fn set_task_endpoint<T: Task>(&mut self, task: &T) {
        let regs = unsafe { &*PPI::ptr() };
        regs.ch[P::CH].tep.write(|w| unsafe { w.bits(task.task_addr().0) });
    }

    fn set_event_endpoint<E: Event>(&mut self, event: &E) {
        let regs = unsafe { &*PPI::ptr() };
        regs.ch[P::CH].eep.write(|w| unsafe { w.bits(event.event_addr().0) });
    }
}

macro_rules! ppi {
    (
        not_fixed: [ $(
            $(#[$attr:meta])*
            ($ppix:ident, $PpixType:ident, $ch:expr),)+
        ],
        fixed: [$(($ppix_fixed:ident, $PpixTypeFixed:ident, $ch_fixed:expr),)+],
    ) => {

        $(
            /// Fully configurable PPI Channel.
            $(#[$attr])*
            pub struct $PpixType {
                _private: (),
            }

            $(#[$attr])*
            impl Channel for $PpixType {
                const CH: usize = $ch;
            }

            $(#[$attr])*
            impl NotFixed for $PpixType {}
        )+

        $(
            /// Fixed PPI channel.
            pub struct $PpixTypeFixed {
                _private: (),
            }

            impl Channel for $PpixTypeFixed {
                const CH: usize = $ch_fixed;
            }
        )+

        /// Type that abstracts all the PPI channels.
        pub struct Parts {
            $(
                $(#[$attr])*
                pub $ppix: $PpixType,
            )+
            $(
                pub $ppix_fixed: $PpixTypeFixed,
            )+
        }

        impl Parts {
            /// Gets access to the PPI abstraction, making it possible to separate the channels through
            /// different objects.
            pub fn new(_regs: PPI) -> Self {
                Self {
                    $(
                        $(#[$attr])*
                        $ppix: $PpixType {
                            _private: (),
                        },
                    )+
                    $(
                        $ppix_fixed: $PpixTypeFixed {
                            _private: (),
                        },
                    )+
                }
            }
        }
    };
}

ppi!(
    not_fixed: [
        (ppi0, Ppi0, 0),
        (ppi1, Ppi1, 1),
        (ppi2, Ppi2, 2),
        (ppi3, Ppi3, 3),
        (ppi4, Ppi4, 4),
        (ppi5, Ppi5, 5),
        (ppi6, Ppi6, 6),
        (ppi7, Ppi7, 7),
        (ppi8, Ppi8, 8),
        (ppi9, Ppi9, 9),
        (ppi10, Ppi10, 10),
        (ppi11, Ppi11, 11),
        (ppi12, Ppi12, 12),
        (ppi13, Ppi13, 13),
        (ppi14, Ppi14, 14),
        (ppi15, Ppi15, 15),
        #[cfg(not(feature = "51"))]
        (ppi16, Ppi16, 16),
        #[cfg(not(feature = "51"))]
        (ppi17, Ppi17, 17),
        #[cfg(not(feature = "51"))]
        (ppi18, Ppi18, 18),
        #[cfg(not(feature = "51"))]
        (ppi19, Ppi19, 19),
    ],
    fixed: [
        (ppi20, Ppi20, 20),
        (ppi21, Ppi21, 21),
        (ppi22, Ppi22, 22),
        (ppi23, Ppi23, 23),
        (ppi24, Ppi24, 24),
        (ppi25, Ppi25, 25),
        (ppi26, Ppi26, 26),
        (ppi27, Ppi27, 27),
        (ppi28, Ppi28, 28),
        (ppi29, Ppi29, 29),
        (ppi30, Ppi30, 30),
        (ppi31, Ppi31, 31),
    ],
);

// Task Impls
//
// To reproduce, in the pac crate, search
//   `rg 'type TASKS_' --type rust`
// Find (regex):
//   `^src/(.*)\.rs:pub type (.*) = .*$`
// Replace (regex):
//   `impl Task for crate::target::$1::$2 { }`
impl Task for crate::target::nfct::TASKS_ACTIVATE { }
impl Task for crate::target::nfct::TASKS_DISABLE { }
impl Task for crate::target::nfct::TASKS_SENSE { }
impl Task for crate::target::nfct::TASKS_STARTTX { }
impl Task for crate::target::nfct::TASKS_ENABLERXDATA { }
impl Task for crate::target::nfct::TASKS_GOIDLE { }
impl Task for crate::target::nfct::TASKS_GOSLEEP { }
impl Task for crate::target::rng::TASKS_START { }
impl Task for crate::target::rng::TASKS_STOP { }
impl Task for crate::target::timer0::TASKS_START { }
impl Task for crate::target::timer0::TASKS_STOP { }
impl Task for crate::target::timer0::TASKS_COUNT { }
impl Task for crate::target::timer0::TASKS_CLEAR { }
impl Task for crate::target::timer0::TASKS_SHUTDOWN { }
impl Task for crate::target::timer0::TASKS_CAPTURE { }
impl Task for crate::target::spis0::TASKS_ACQUIRE { }
impl Task for crate::target::spis0::TASKS_RELEASE { }
impl Task for crate::target::uart0::TASKS_STARTRX { }
impl Task for crate::target::uart0::TASKS_STOPRX { }
impl Task for crate::target::uart0::TASKS_STARTTX { }
impl Task for crate::target::uart0::TASKS_STOPTX { }
impl Task for crate::target::uart0::TASKS_SUSPEND { }
impl Task for crate::target::gpiote::TASKS_OUT { }
impl Task for crate::target::gpiote::TASKS_SET { }
impl Task for crate::target::gpiote::TASKS_CLR { }
impl Task for crate::target::clock::TASKS_HFCLKSTART { }
impl Task for crate::target::clock::TASKS_HFCLKSTOP { }
impl Task for crate::target::clock::TASKS_LFCLKSTART { }
impl Task for crate::target::clock::TASKS_LFCLKSTOP { }
impl Task for crate::target::clock::TASKS_CAL { }
impl Task for crate::target::clock::TASKS_CTSTART { }
impl Task for crate::target::clock::TASKS_CTSTOP { }
impl Task for crate::target::spim0::TASKS_START { }
impl Task for crate::target::spim0::TASKS_STOP { }
impl Task for crate::target::spim0::TASKS_SUSPEND { }
impl Task for crate::target::spim0::TASKS_RESUME { }
impl Task for crate::target::power::TASKS_CONSTLAT { }
impl Task for crate::target::power::TASKS_LOWPWR { }
impl Task for crate::target::twim0::TASKS_STARTRX { }
impl Task for crate::target::twim0::TASKS_STARTTX { }
impl Task for crate::target::twim0::TASKS_STOP { }
impl Task for crate::target::twim0::TASKS_SUSPEND { }
impl Task for crate::target::twim0::TASKS_RESUME { }
impl Task for crate::target::twi0::TASKS_STARTRX { }
impl Task for crate::target::twi0::TASKS_STARTTX { }
impl Task for crate::target::twi0::TASKS_STOP { }
impl Task for crate::target::twi0::TASKS_SUSPEND { }
impl Task for crate::target::twi0::TASKS_RESUME { }
impl Task for crate::target::egu0::TASKS_TRIGGER { }
impl Task for crate::target::ecb::TASKS_STARTECB { }
impl Task for crate::target::ecb::TASKS_STOPECB { }
impl Task for crate::target::wdt::TASKS_START { }
impl Task for crate::target::pdm::TASKS_START { }
impl Task for crate::target::pdm::TASKS_STOP { }
impl Task for crate::target::rtc0::TASKS_START { }
impl Task for crate::target::rtc0::TASKS_STOP { }
impl Task for crate::target::rtc0::TASKS_CLEAR { }
impl Task for crate::target::rtc0::TASKS_TRIGOVRFLW { }
impl Task for crate::target::lpcomp::TASKS_START { }
impl Task for crate::target::lpcomp::TASKS_STOP { }
impl Task for crate::target::lpcomp::TASKS_SAMPLE { }
impl Task for crate::target::radio::TASKS_TXEN { }
impl Task for crate::target::radio::TASKS_RXEN { }
impl Task for crate::target::radio::TASKS_START { }
impl Task for crate::target::radio::TASKS_STOP { }
impl Task for crate::target::radio::TASKS_DISABLE { }
impl Task for crate::target::radio::TASKS_RSSISTART { }
impl Task for crate::target::radio::TASKS_RSSISTOP { }
impl Task for crate::target::radio::TASKS_BCSTART { }
impl Task for crate::target::radio::TASKS_BCSTOP { }
impl Task for crate::target::temp::TASKS_START { }
impl Task for crate::target::temp::TASKS_STOP { }
impl Task for crate::target::ccm::TASKS_KSGEN { }
impl Task for crate::target::ccm::TASKS_CRYPT { }
impl Task for crate::target::ccm::TASKS_STOP { }
impl Task for crate::target::uarte0::TASKS_STARTRX { }
impl Task for crate::target::uarte0::TASKS_STOPRX { }
impl Task for crate::target::uarte0::TASKS_STARTTX { }
impl Task for crate::target::uarte0::TASKS_STOPTX { }
impl Task for crate::target::uarte0::TASKS_FLUSHRX { }
impl Task for crate::target::i2s::TASKS_START { }
impl Task for crate::target::i2s::TASKS_STOP { }
impl Task for crate::target::twis0::TASKS_STOP { }
impl Task for crate::target::twis0::TASKS_SUSPEND { }
impl Task for crate::target::twis0::TASKS_RESUME { }
impl Task for crate::target::twis0::TASKS_PREPARERX { }
impl Task for crate::target::twis0::TASKS_PREPARETX { }
impl Task for crate::target::timer3::TASKS_START { }
impl Task for crate::target::timer3::TASKS_STOP { }
impl Task for crate::target::timer3::TASKS_COUNT { }
impl Task for crate::target::timer3::TASKS_CLEAR { }
impl Task for crate::target::timer3::TASKS_SHUTDOWN { }
impl Task for crate::target::timer3::TASKS_CAPTURE { }
impl Task for crate::target::qdec::TASKS_START { }
impl Task for crate::target::qdec::TASKS_STOP { }
impl Task for crate::target::qdec::TASKS_READCLRACC { }
impl Task for crate::target::qdec::TASKS_RDCLRACC { }
impl Task for crate::target::qdec::TASKS_RDCLRDBL { }
impl Task for crate::target::aar::TASKS_START { }
impl Task for crate::target::aar::TASKS_STOP { }
impl Task for crate::target::comp::TASKS_START { }
impl Task for crate::target::comp::TASKS_STOP { }
impl Task for crate::target::comp::TASKS_SAMPLE { }
impl Task for crate::target::saadc::TASKS_START { }
impl Task for crate::target::saadc::TASKS_SAMPLE { }
impl Task for crate::target::saadc::TASKS_STOP { }
impl Task for crate::target::saadc::TASKS_CALIBRATEOFFSET { }
impl Task for crate::target::pwm0::TASKS_STOP { }
impl Task for crate::target::pwm0::TASKS_SEQSTART { }
impl Task for crate::target::pwm0::TASKS_NEXTSTEP { }

// Event impls
//
// To reproduce, in the pac crate, search
//   `rg 'type EVENTS_' --type rust`
// Find (regex):
//   `^src/(.*)\.rs:pub type (.*) = .*$`
// Replace (regex):
//   `impl Event for crate::target::$1::$2 { }`
impl Event for crate::target::rng::EVENTS_VALRDY { }
impl Event for crate::target::timer0::EVENTS_COMPARE { }
impl Event for crate::target::uart0::EVENTS_CTS { }
impl Event for crate::target::uart0::EVENTS_NCTS { }
impl Event for crate::target::uart0::EVENTS_RXDRDY { }
impl Event for crate::target::uart0::EVENTS_TXDRDY { }
impl Event for crate::target::uart0::EVENTS_ERROR { }
impl Event for crate::target::uart0::EVENTS_RXTO { }
impl Event for crate::target::spim0::EVENTS_STOPPED { }
impl Event for crate::target::spim0::EVENTS_ENDRX { }
impl Event for crate::target::spim0::EVENTS_END { }
impl Event for crate::target::spim0::EVENTS_ENDTX { }
impl Event for crate::target::spim0::EVENTS_STARTED { }
impl Event for crate::target::spis0::EVENTS_END { }
impl Event for crate::target::spis0::EVENTS_ENDRX { }
impl Event for crate::target::spis0::EVENTS_ACQUIRED { }
impl Event for crate::target::gpiote::EVENTS_IN { }
impl Event for crate::target::gpiote::EVENTS_PORT { }
impl Event for crate::target::clock::EVENTS_HFCLKSTARTED { }
impl Event for crate::target::clock::EVENTS_LFCLKSTARTED { }
impl Event for crate::target::clock::EVENTS_DONE { }
impl Event for crate::target::clock::EVENTS_CTTO { }
impl Event for crate::target::power::EVENTS_POFWARN { }
impl Event for crate::target::power::EVENTS_SLEEPENTER { }
impl Event for crate::target::power::EVENTS_SLEEPEXIT { }
impl Event for crate::target::spi0::EVENTS_READY { }
impl Event for crate::target::twim0::EVENTS_STOPPED { }
impl Event for crate::target::twim0::EVENTS_ERROR { }
impl Event for crate::target::twim0::EVENTS_SUSPENDED { }
impl Event for crate::target::twim0::EVENTS_RXSTARTED { }
impl Event for crate::target::twim0::EVENTS_TXSTARTED { }
impl Event for crate::target::twim0::EVENTS_LASTRX { }
impl Event for crate::target::twim0::EVENTS_LASTTX { }
impl Event for crate::target::egu0::EVENTS_TRIGGERED { }
impl Event for crate::target::wdt::EVENTS_TIMEOUT { }
impl Event for crate::target::twi0::EVENTS_STOPPED { }
impl Event for crate::target::twi0::EVENTS_RXDREADY { }
impl Event for crate::target::twi0::EVENTS_TXDSENT { }
impl Event for crate::target::twi0::EVENTS_ERROR { }
impl Event for crate::target::twi0::EVENTS_BB { }
impl Event for crate::target::twi0::EVENTS_SUSPENDED { }
impl Event for crate::target::pdm::EVENTS_STARTED { }
impl Event for crate::target::pdm::EVENTS_STOPPED { }
impl Event for crate::target::pdm::EVENTS_END { }
impl Event for crate::target::ecb::EVENTS_ENDECB { }
impl Event for crate::target::ecb::EVENTS_ERRORECB { }
impl Event for crate::target::rtc0::EVENTS_TICK { }
impl Event for crate::target::rtc0::EVENTS_OVRFLW { }
impl Event for crate::target::rtc0::EVENTS_COMPARE { }
impl Event for crate::target::lpcomp::EVENTS_READY { }
impl Event for crate::target::lpcomp::EVENTS_DOWN { }
impl Event for crate::target::lpcomp::EVENTS_UP { }
impl Event for crate::target::lpcomp::EVENTS_CROSS { }
impl Event for crate::target::radio::EVENTS_READY { }
impl Event for crate::target::radio::EVENTS_ADDRESS { }
impl Event for crate::target::radio::EVENTS_PAYLOAD { }
impl Event for crate::target::radio::EVENTS_END { }
impl Event for crate::target::radio::EVENTS_DISABLED { }
impl Event for crate::target::radio::EVENTS_DEVMATCH { }
impl Event for crate::target::radio::EVENTS_DEVMISS { }
impl Event for crate::target::radio::EVENTS_RSSIEND { }
impl Event for crate::target::radio::EVENTS_BCMATCH { }
impl Event for crate::target::radio::EVENTS_CRCOK { }
impl Event for crate::target::radio::EVENTS_CRCERROR { }
impl Event for crate::target::temp::EVENTS_DATARDY { }
impl Event for crate::target::ccm::EVENTS_ENDKSGEN { }
impl Event for crate::target::ccm::EVENTS_ENDCRYPT { }
impl Event for crate::target::ccm::EVENTS_ERROR { }
impl Event for crate::target::i2s::EVENTS_RXPTRUPD { }
impl Event for crate::target::i2s::EVENTS_STOPPED { }
impl Event for crate::target::i2s::EVENTS_TXPTRUPD { }
impl Event for crate::target::uarte0::EVENTS_CTS { }
impl Event for crate::target::uarte0::EVENTS_NCTS { }
impl Event for crate::target::uarte0::EVENTS_RXDRDY { }
impl Event for crate::target::uarte0::EVENTS_ENDRX { }
impl Event for crate::target::uarte0::EVENTS_TXDRDY { }
impl Event for crate::target::uarte0::EVENTS_ENDTX { }
impl Event for crate::target::uarte0::EVENTS_ERROR { }
impl Event for crate::target::uarte0::EVENTS_RXTO { }
impl Event for crate::target::uarte0::EVENTS_RXSTARTED { }
impl Event for crate::target::uarte0::EVENTS_TXSTARTED { }
impl Event for crate::target::uarte0::EVENTS_TXSTOPPED { }
impl Event for crate::target::twis0::EVENTS_STOPPED { }
impl Event for crate::target::twis0::EVENTS_ERROR { }
impl Event for crate::target::twis0::EVENTS_RXSTARTED { }
impl Event for crate::target::twis0::EVENTS_TXSTARTED { }
impl Event for crate::target::twis0::EVENTS_WRITE { }
impl Event for crate::target::twis0::EVENTS_READ { }
impl Event for crate::target::timer3::EVENTS_COMPARE { }
impl Event for crate::target::qdec::EVENTS_SAMPLERDY { }
impl Event for crate::target::qdec::EVENTS_REPORTRDY { }
impl Event for crate::target::qdec::EVENTS_ACCOF { }
impl Event for crate::target::qdec::EVENTS_DBLRDY { }
impl Event for crate::target::qdec::EVENTS_STOPPED { }
impl Event for crate::target::aar::EVENTS_END { }
impl Event for crate::target::aar::EVENTS_RESOLVED { }
impl Event for crate::target::aar::EVENTS_NOTRESOLVED { }
impl Event for crate::target::saadc::EVENTS_STARTED { }
impl Event for crate::target::saadc::EVENTS_END { }
impl Event for crate::target::saadc::EVENTS_DONE { }
impl Event for crate::target::saadc::EVENTS_RESULTDONE { }
impl Event for crate::target::saadc::EVENTS_CALIBRATEDONE { }
impl Event for crate::target::saadc::EVENTS_STOPPED { }
impl Event for crate::target::comp::EVENTS_READY { }
impl Event for crate::target::comp::EVENTS_DOWN { }
impl Event for crate::target::comp::EVENTS_UP { }
impl Event for crate::target::comp::EVENTS_CROSS { }
impl Event for crate::target::pwm0::EVENTS_STOPPED { }
impl Event for crate::target::pwm0::EVENTS_SEQSTARTED { }
impl Event for crate::target::pwm0::EVENTS_SEQEND { }
impl Event for crate::target::pwm0::EVENTS_PWMPERIODEND { }
impl Event for crate::target::pwm0::EVENTS_LOOPSDONE { }
impl Event for crate::target::nfct::EVENTS_READY { }
impl Event for crate::target::nfct::EVENTS_FIELDDETECTED { }
impl Event for crate::target::nfct::EVENTS_FIELDLOST { }
impl Event for crate::target::nfct::EVENTS_TXFRAMESTART { }
impl Event for crate::target::nfct::EVENTS_TXFRAMEEND { }
impl Event for crate::target::nfct::EVENTS_RXFRAMESTART { }
impl Event for crate::target::nfct::EVENTS_RXFRAMEEND { }
impl Event for crate::target::nfct::EVENTS_ERROR { }
impl Event for crate::target::nfct::EVENTS_RXERROR { }
impl Event for crate::target::nfct::EVENTS_ENDRX { }
impl Event for crate::target::nfct::EVENTS_ENDTX { }
impl Event for crate::target::nfct::EVENTS_AUTOCOLRESSTARTED { }
impl Event for crate::target::nfct::EVENTS_COLLISION { }
impl Event for crate::target::nfct::EVENTS_SELECTED { }
impl Event for crate::target::nfct::EVENTS_STARTED { }
