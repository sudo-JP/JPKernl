//! Context switching for RP2040 (Cortex-M0+)
//!
//! Stack layout for all processes (compatible with ISR):
//!   [High address]
//!   xPSR, PC, LR, R12, R3, R2, R1, R0   <- exception frame (8 words)
//!   R11, R10, R9, R8, R7, R6, R5, R4    <- saved registers (8 words)
//!   [Low address] <- SP points here
//!
//! Total: 16 words = 64 bytes

use crate::{Scheduler, CURRENT, PROCS, SCHEDULER};

/// Switch context from ISR (timer interrupt or PendSV)
/// Hardware has already pushed exception frame to PSP.
/// We save R4-R11 additionally, switch stacks, restore R4-R11, return via EXC_RETURN.
///
/// r0 = pointer to store old SP
/// r1 = new SP to load
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn switch_context_isr(old_sp_ptr: *mut *mut u32, new_sp: *const u32) {
    core::arch::naked_asm!(
        // === Save current process ===
        // Get PSP (points to exception frame that hardware pushed)
        "mrs r2, psp",
        
        // Reserve space for R4-R11 (8 regs * 4 bytes = 32)
        "subs r2, r2, #32",
        
        // Save R4-R7 at [r2]
        "str r4, [r2, #0]",
        "str r5, [r2, #4]",
        "str r6, [r2, #8]",
        "str r7, [r2, #12]",
        
        // Save R8-R11 at [r2+16]
        "mov r3, r8",
        "str r3, [r2, #16]",
        "mov r3, r9",
        "str r3, [r2, #20]",
        "mov r3, r10",
        "str r3, [r2, #24]",
        "mov r3, r11",
        "str r3, [r2, #28]",
        
        // Store SP to *old_sp_ptr
        "str r2, [r0]",
        
        // === Load new process ===
        // r1 = new SP (points to R4)
        
        // Restore R4-R7
        "ldr r4, [r1, #0]",
        "ldr r5, [r1, #4]",
        "ldr r6, [r1, #8]",
        "ldr r7, [r1, #12]",
        
        // Restore R8-R11
        "ldr r3, [r1, #16]",
        "mov r8, r3",
        "ldr r3, [r1, #20]",
        "mov r9, r3",
        "ldr r3, [r1, #24]",
        "mov r10, r3",
        "ldr r3, [r1, #28]",
        "mov r11, r3",
        
        // Set PSP to point past R4-R11 (to exception frame)
        "adds r1, r1, #32",
        "msr psp, r1",
        
        // Return to thread mode using PSP
        "ldr r0, =0xFFFFFFFD",
        "bx r0",
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
        run_first_process(process.sp);
    }
}

/*
 * asm bootstrap to get first process started 
 *
 * Since we currently running kernel code, 
 * and want to jump to process code, we restore the 
 * first process registers, then run the process code.
 *
 * */
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn run_first_process(sp: *const u32) -> ! {
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
    )
}

