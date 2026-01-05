use crate::{Scheduler, CURRENT, PCB, PROCS, SCHEDULER};

pub fn switch_context(old_pcb: &mut PCB, new_pcb: &PCB) -> () {
    unsafe {
        let old_sp_ptr: *mut *mut u32 = &mut old_pcb.sp;
        let new_sp: *const u32 = new_pcb.sp; 

        getcontext(old_sp_ptr);   
        setcontext(new_sp);        
    } 
}

/*
 * Called during Interrupt Service Routine
 * Literally getcontext from C
 * */
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getcontext(old_sp_ptr: *mut *mut u32) {
    //, new_sp: *const u32) {
    core::arch::naked_asm!(
        // Allocate stack downward, then access them like an array 
        // We want to manually save r4-r11, so 8 words * 4 = 32 down
        "mrs r2, psp", // Current process sp 
        
        // For some reason i cant sub by 32 
        "subs r2, r2, #32",

        "str r4, [r2, #0]", 
        "str r5, [r2, #4]", 
        "str r6, [r2, #8]", 
        "str r7, [r2, #12]",

        // Temp registers 
        "mov r4, r8", 
        "str r4, [r2, #16]", 
        "mov r5, r9",
        "str r5, [r2, #20]",
        "mov r6, r10", 
        "str r6, [r2, #24]", 
        "mov r7, r11",
        "str r7, [r2, #28]",

        // Store new stack to r0 
        "str r2, [r0]",
    );
}

/*
 * Function should never return, call to run first process given the sp 
 * */
pub fn start_first_process() -> () {
    let sched = core::ptr::addr_of_mut!(SCHEDULER); 
    unsafe {
        let pid = (*sched).dequeue().unwrap();
        let process = PROCS[pid as usize].unwrap();
        CURRENT = Some(pid);

        // This function should not return 
        setcontext(process.sp);
    }

    #[allow(unreachable_code)]
    { panic!("Code should not reach here"); }
}

/*
 *
 * Since we currently running kernel code, 
 * and want to jump to process code, we restore the 
 * first process registers, then run the process code.
 *
 * Literally setcontext from C 
 * */
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn setcontext(sp: *const u32) -> ! {
    core::arch::naked_asm!(
        // Restore r4-r7 registers from SP (from function arg at r0)
        "ldr r4, [r0, #0]", 
        "ldr r5, [r0, #4]", 
        "ldr r6, [r0, #8]", 
        "ldr r7, [r0, #12]", 

        // Can't directly do ldr on r8-r11 because thumb only, whatever that means
        // We use r1 as temporary reg instead
        "ldr r1, [r0, #16]", 
        "mov r8, r1", 
        "ldr r1, [r0, #20]", 
        "mov r9, r1",
        "ldr r1, [r0, #24]", 
        "mov r10, r1",
        "ldr r1, [r0, #28]", 
        "mov r11, r1",

        // Advance sp to point to r0 
        "adds r0, r0, #32", 

        // Set the process sp to r0
        "msr psp, r0", 

        // Switch to thread mode by writing 1 to control reg
        // Add on bit index 1 (which is 2)
        "movs r1, #2",

        // Save back to control
        "msr CONTROL, r1", 

        // Sync barrier
        "isb", 

        // Restore special registers, pop from sp to r0-r3
        // Since in thread mode, and we set sp to psp, this is valid
        "pop {{r0-r3}}",
        "pop {{r4}}", 
        "mov r12, r4",

        "pop {{r4}}",       // LR  
        "mov lr, r4",

        "pop {{r4, r5}}",   // PC lives in r4 temporarily, discard r5

        "bx r4",
        ""
    );
}

