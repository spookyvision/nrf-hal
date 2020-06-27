use crate::ppi::Task;

// Task Impls
//
// To reproduce, in the pac crate, search
//   `rg 'type TASKS_.*crate::Reg' --type rust`
// Find (regex):
//   `^src/(.*)\.rs:pub type (.*) = .*$`
// Replace (regex):
//   `impl Task for crate::target::$1::$2 { }`
impl Task for crate::target::ipc_ns::TASKS_SEND {}
impl Task for crate::target::i2s_ns::TASKS_START {}
impl Task for crate::target::i2s_ns::TASKS_STOP {}
impl Task for crate::target::twim0_ns::TASKS_STARTRX {}
impl Task for crate::target::twim0_ns::TASKS_STARTTX {}
impl Task for crate::target::twim0_ns::TASKS_STOP {}
impl Task for crate::target::twim0_ns::TASKS_SUSPEND {}
impl Task for crate::target::twim0_ns::TASKS_RESUME {}
impl Task for crate::target::timer0_ns::TASKS_START {}
impl Task for crate::target::timer0_ns::TASKS_STOP {}
impl Task for crate::target::timer0_ns::TASKS_COUNT {}
impl Task for crate::target::timer0_ns::TASKS_CLEAR {}
impl Task for crate::target::timer0_ns::TASKS_SHUTDOWN {}
impl Task for crate::target::timer0_ns::TASKS_CAPTURE {}
impl Task for crate::target::twis0_ns::TASKS_STOP {}
impl Task for crate::target::twis0_ns::TASKS_SUSPEND {}
impl Task for crate::target::twis0_ns::TASKS_RESUME {}
impl Task for crate::target::twis0_ns::TASKS_PREPARERX {}
impl Task for crate::target::twis0_ns::TASKS_PREPARETX {}
impl Task for crate::target::pdm_ns::TASKS_START {}
impl Task for crate::target::pdm_ns::TASKS_STOP {}
impl Task for crate::target::rtc0_ns::TASKS_START {}
impl Task for crate::target::rtc0_ns::TASKS_STOP {}
impl Task for crate::target::rtc0_ns::TASKS_CLEAR {}
impl Task for crate::target::rtc0_ns::TASKS_TRIGOVRFLW {}
impl Task for crate::target::gpiote0_s::TASKS_OUT {}
impl Task for crate::target::gpiote0_s::TASKS_SET {}
impl Task for crate::target::gpiote0_s::TASKS_CLR {}
impl Task for crate::target::saadc_ns::TASKS_START {}
impl Task for crate::target::saadc_ns::TASKS_SAMPLE {}
impl Task for crate::target::saadc_ns::TASKS_STOP {}
impl Task for crate::target::saadc_ns::TASKS_CALIBRATEOFFSET {}
impl Task for crate::target::spim0_ns::TASKS_START {}
impl Task for crate::target::spim0_ns::TASKS_STOP {}
impl Task for crate::target::spim0_ns::TASKS_SUSPEND {}
impl Task for crate::target::spim0_ns::TASKS_RESUME {}
impl Task for crate::target::egu0_ns::TASKS_TRIGGER {}
impl Task for crate::target::kmu_ns::TASKS_PUSH_KEYSLOT {}
impl Task for crate::target::spis0_ns::TASKS_ACQUIRE {}
impl Task for crate::target::spis0_ns::TASKS_RELEASE {}
impl Task for crate::target::pwm0_ns::TASKS_STOP {}
impl Task for crate::target::pwm0_ns::TASKS_SEQSTART {}
impl Task for crate::target::pwm0_ns::TASKS_NEXTSTEP {}
impl Task for crate::target::power_ns::TASKS_CONSTLAT {}
impl Task for crate::target::power_ns::TASKS_LOWPWR {}
impl Task for crate::target::uarte0_ns::TASKS_STARTRX {}
impl Task for crate::target::uarte0_ns::TASKS_STOPRX {}
impl Task for crate::target::uarte0_ns::TASKS_STARTTX {}
impl Task for crate::target::uarte0_ns::TASKS_STOPTX {}
impl Task for crate::target::uarte0_ns::TASKS_FLUSHRX {}
impl Task for crate::target::clock_ns::TASKS_HFCLKSTART {}
impl Task for crate::target::clock_ns::TASKS_HFCLKSTOP {}
impl Task for crate::target::clock_ns::TASKS_LFCLKSTART {}
impl Task for crate::target::clock_ns::TASKS_LFCLKSTOP {}
impl Task for crate::target::wdt_ns::TASKS_START {}
