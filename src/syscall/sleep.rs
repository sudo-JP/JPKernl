use crate::{get_time_us, yield_now, BlockReason, Scheduler, SchedulerError, SleepEntry, CURRENT, PROCS, SLEEP_QUEUE};

pub fn sleep_ms(ms: u32) -> Result<(), SchedulerError> {
    let wake_time = get_time_us() + (ms as u64 * 1000);
    unsafe {
        let pid = CURRENT.ok_or(SchedulerError::NoCurrent)?;
        let entry = SleepEntry{
            pid: pid, 
            wake_time: wake_time,
        };

        let q = core::ptr::addr_of_mut!(SLEEP_QUEUE);
        (*q).enqueue(entry)?;

        PROCS[pid as usize].as_mut().unwrap().state =
            crate::ProcessState::Blocked(BlockReason::Sleeping(wake_time));

        yield_now()?;
    }

    Ok(())
}
