use cortex_m::interrupt::Mutex;
use cortex_m_rt::exception;
use rp2040_hal::{timer::{Alarm, Alarm0}};
use core::cell::RefCell;
use rp2040_hal::pac::interrupt;
use core::ptr;

use crate::{check_sleep_and_wake, scheduler::{CURRENT, PROCS, SCHEDULER}, switch_context, Scheduler, PCB, QUANTUM, SLEEP_QUEUE};


static ALARM: Mutex<RefCell<Option<Alarm0>>> = Mutex::new(RefCell::new(None));

pub fn set_alarm(alarm: Alarm0) {
    cortex_m::interrupt::free(|cs| {
        ALARM.borrow(cs).replace(Some(alarm));
    });
}

// Clear and reschedule the timer alarm
fn handle_alarm() {
    cortex_m::interrupt::free(|cs| {
        if let Some(ref mut alarm) = ALARM.borrow(cs).borrow_mut().as_mut() {
            alarm.clear_interrupt();
            let _ = alarm.schedule(QUANTUM);
        }
    });
}



#[interrupt]
unsafe fn TIMER_IRQ_0() -> ! {
    handle_alarm();  

    let sched = ptr::addr_of_mut!(SCHEDULER);
    let sleep_q = ptr::addr_of_mut!(SLEEP_QUEUE);
    
    unsafe {
        let old_pid = CURRENT.unwrap();

        // wake up all sleeping processes 
        while (*sleep_q).get_size() > 0 {
            if check_sleep_and_wake().is_err() {
                break; 
            }
        }

        // Get new process
        let next_pid = (*sched).dequeue().ok().unwrap();

        let old_pcb: *mut PCB = PROCS[old_pid as usize].as_mut().unwrap();
        match (*old_pcb).state {
            crate::ProcessState::Ready | crate::ProcessState::Running => {
                let _ = (*sched).enqueue(old_pid);
            }
            _ => {},
        }
        
        if old_pid == next_pid {
            switch_context(&mut (*old_pcb), &(*old_pcb));
        }
        
        CURRENT = Some(next_pid);
        
        let new_pcb: *mut PCB = PROCS[next_pid as usize].as_mut().unwrap();
        
        (*old_pcb).state = crate::ProcessState::Ready;
        (*new_pcb).state = crate::ProcessState::Running;
        
        switch_context(&mut (*old_pcb), &(*new_pcb));
    }
    loop {}
}

#[exception]
unsafe fn PendSV() -> ! {
    let sched = ptr::addr_of_mut!(SCHEDULER);
    let sleep_q = ptr::addr_of_mut!(SLEEP_QUEUE);
    
    unsafe {
        let old_pid = CURRENT.unwrap();

        // wake up all sleeping processes 
        while (*sleep_q).get_size() > 0 {
            if check_sleep_and_wake().is_err() {
                break; 
            }
        }

        // Get new process
        let next_pid = (*sched).dequeue().ok().unwrap();

        let old_pcb: *mut PCB = PROCS[old_pid as usize].as_mut().unwrap();
        match (*old_pcb).state {
            crate::ProcessState::Ready | crate::ProcessState::Running => {
                let _ = (*sched).enqueue(old_pid);
            }
            _ => {},
        }
        let _ = (*sched).enqueue(old_pid);
        
        if old_pid == next_pid {
            switch_context(&mut (*old_pcb), &(*old_pcb));
        }
        
        CURRENT = Some(next_pid);
        
        let new_pcb: *mut PCB = PROCS[next_pid as usize].as_mut().unwrap();
        
        (*old_pcb).state = crate::ProcessState::Ready;
        (*new_pcb).state = crate::ProcessState::Running;
        
        switch_context(&mut (*old_pcb), &(*new_pcb));
    }
    loop {}
}
