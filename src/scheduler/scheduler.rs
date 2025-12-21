use core::ptr;
use crate::{scheduler::{CURRENT, PROCS, SCHEDULER}, switch_context, PCB};

#[derive(Debug)]
pub enum SchedulerError {
    NoSpace, 
    Empty, 
    NoCurrent, 
    ProcessNotFound,
}

pub trait Scheduler {
    fn enqueue(&mut self, pid: u8) -> Result<(), SchedulerError>; 
    fn dequeue(&mut self) -> Result<u8, SchedulerError>;  
}

pub fn current() -> Option<u8> {
    unsafe { CURRENT }
}

pub fn yield_now() -> Result<(), SchedulerError> {
    unsafe {
        let sched = ptr::addr_of_mut!(SCHEDULER); 

        let old_pid = CURRENT.ok_or(SchedulerError::NoCurrent)?;
        let next_pid = (*sched).dequeue()?;

        // Context switching back to itself
        if old_pid == next_pid {
            (*sched).enqueue(old_pid)?;
            return Ok(());
        }

        (*sched).enqueue(old_pid)?;
        CURRENT = Some(next_pid);
        
    let old_pcb: *mut PCB = PROCS[old_pid as usize]
        .as_mut()
        .ok_or(SchedulerError::ProcessNotFound)?;

    let new_pcb: *const PCB = PROCS[next_pid as usize]
        .as_ref()
        .ok_or(SchedulerError::ProcessNotFound)?;

        switch_context(old_pcb, new_pcb);

    }

    Ok(())
}
