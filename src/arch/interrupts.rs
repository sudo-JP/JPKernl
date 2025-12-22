use cortex_m::interrupt::{Mutex};
use rp2040_hal::{fugit::ExtU32, timer::{Alarm, Alarm0}};
use core::cell::RefCell;
use rp2040_hal::pac::interrupt;

use crate::yield_now;


static ALARM: Mutex<RefCell<Option<Alarm0>>> = Mutex::new(RefCell::new(None));

pub fn set_alarm(alarm: Alarm0) {
    cortex_m::interrupt::free(|cs| {
        ALARM.borrow(cs).replace(Some(alarm));
    });
}

#[interrupt]
fn TIMER_IRQ_0() {
    cortex_m::interrupt::free(|cs| {
        if let Some(ref mut alarm) = 
            ALARM.borrow(cs)
                .borrow_mut()
                .as_mut() {
            alarm.clear_interrupt();
            let _ = alarm.schedule(100_000u32.micros()); // 10 micros sec 
        }
    });

    let _ = yield_now();
}
