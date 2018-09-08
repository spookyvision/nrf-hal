use hal::nrf52840::TEMP;

pub fn read_temperature(temp: &TEMP) -> f32
{
    unsafe { temp.tasks_start.write(|w| w.bits(1)); }
    while temp.events_datardy.read().bits() == 0 {}
    (temp.temp.read().bits() as f32) * 0.25
}
