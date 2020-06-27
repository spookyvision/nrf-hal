use crate::ppi::Event;

// Event impls
//
// To reproduce, in the pac crate, search
//   `rg 'type EVENTS_.*crate::Reg' --type rust`
// Find (regex):
//   `^src/(.*)\.rs:pub type (.*) = .*$`
// Replace (regex):
//   `impl Event for crate::target::$1::$2 { }`
impl Event for crate::target::ipc_ns::EVENTS_RECEIVE {}
impl Event for crate::target::i2s_ns::EVENTS_RXPTRUPD {}
impl Event for crate::target::i2s_ns::EVENTS_STOPPED {}
impl Event for crate::target::i2s_ns::EVENTS_TXPTRUPD {}
impl Event for crate::target::twim0_ns::EVENTS_STOPPED {}
impl Event for crate::target::twim0_ns::EVENTS_ERROR {}
impl Event for crate::target::twim0_ns::EVENTS_SUSPENDED {}
impl Event for crate::target::twim0_ns::EVENTS_RXSTARTED {}
impl Event for crate::target::twim0_ns::EVENTS_TXSTARTED {}
impl Event for crate::target::twim0_ns::EVENTS_LASTRX {}
impl Event for crate::target::twim0_ns::EVENTS_LASTTX {}
impl Event for crate::target::timer0_ns::EVENTS_COMPARE {}
impl Event for crate::target::twis0_ns::EVENTS_STOPPED {}
impl Event for crate::target::twis0_ns::EVENTS_ERROR {}
impl Event for crate::target::twis0_ns::EVENTS_RXSTARTED {}
impl Event for crate::target::twis0_ns::EVENTS_TXSTARTED {}
impl Event for crate::target::twis0_ns::EVENTS_WRITE {}
impl Event for crate::target::twis0_ns::EVENTS_READ {}
impl Event for crate::target::pdm_ns::EVENTS_STARTED {}
impl Event for crate::target::pdm_ns::EVENTS_STOPPED {}
impl Event for crate::target::pdm_ns::EVENTS_END {}
impl Event for crate::target::rtc0_ns::EVENTS_TICK {}
impl Event for crate::target::rtc0_ns::EVENTS_OVRFLW {}
impl Event for crate::target::rtc0_ns::EVENTS_COMPARE {}
impl Event for crate::target::spu_s::EVENTS_RAMACCERR {}
impl Event for crate::target::spu_s::EVENTS_FLASHACCERR {}
impl Event for crate::target::spu_s::EVENTS_PERIPHACCERR {}
impl Event for crate::target::gpiote0_s::EVENTS_IN {}
impl Event for crate::target::gpiote0_s::EVENTS_PORT {}
impl Event for crate::target::saadc_ns::EVENTS_STARTED {}
impl Event for crate::target::saadc_ns::EVENTS_END {}
impl Event for crate::target::saadc_ns::EVENTS_DONE {}
impl Event for crate::target::saadc_ns::EVENTS_RESULTDONE {}
impl Event for crate::target::saadc_ns::EVENTS_CALIBRATEDONE {}
impl Event for crate::target::saadc_ns::EVENTS_STOPPED {}
impl Event for crate::target::egu0_ns::EVENTS_TRIGGERED {}
impl Event for crate::target::spim0_ns::EVENTS_STOPPED {}
impl Event for crate::target::spim0_ns::EVENTS_ENDRX {}
impl Event for crate::target::spim0_ns::EVENTS_END {}
impl Event for crate::target::spim0_ns::EVENTS_ENDTX {}
impl Event for crate::target::spim0_ns::EVENTS_STARTED {}
impl Event for crate::target::spis0_ns::EVENTS_END {}
impl Event for crate::target::spis0_ns::EVENTS_ENDRX {}
impl Event for crate::target::spis0_ns::EVENTS_ACQUIRED {}
impl Event for crate::target::kmu_ns::EVENTS_KEYSLOT_PUSHED {}
impl Event for crate::target::kmu_ns::EVENTS_KEYSLOT_REVOKED {}
impl Event for crate::target::kmu_ns::EVENTS_KEYSLOT_ERROR {}
impl Event for crate::target::power_ns::EVENTS_POFWARN {}
impl Event for crate::target::power_ns::EVENTS_SLEEPENTER {}
impl Event for crate::target::power_ns::EVENTS_SLEEPEXIT {}
impl Event for crate::target::pwm0_ns::EVENTS_STOPPED {}
impl Event for crate::target::pwm0_ns::EVENTS_SEQSTARTED {}
impl Event for crate::target::pwm0_ns::EVENTS_SEQEND {}
impl Event for crate::target::pwm0_ns::EVENTS_PWMPERIODEND {}
impl Event for crate::target::pwm0_ns::EVENTS_LOOPSDONE {}
impl Event for crate::target::clock_ns::EVENTS_HFCLKSTARTED {}
impl Event for crate::target::clock_ns::EVENTS_LFCLKSTARTED {}
impl Event for crate::target::uarte0_ns::EVENTS_CTS {}
impl Event for crate::target::uarte0_ns::EVENTS_NCTS {}
impl Event for crate::target::uarte0_ns::EVENTS_RXDRDY {}
impl Event for crate::target::uarte0_ns::EVENTS_ENDRX {}
impl Event for crate::target::uarte0_ns::EVENTS_TXDRDY {}
impl Event for crate::target::uarte0_ns::EVENTS_ENDTX {}
impl Event for crate::target::uarte0_ns::EVENTS_ERROR {}
impl Event for crate::target::uarte0_ns::EVENTS_RXTO {}
impl Event for crate::target::uarte0_ns::EVENTS_RXSTARTED {}
impl Event for crate::target::uarte0_ns::EVENTS_TXSTARTED {}
impl Event for crate::target::uarte0_ns::EVENTS_TXSTOPPED {}
impl Event for crate::target::wdt_ns::EVENTS_TIMEOUT {}
