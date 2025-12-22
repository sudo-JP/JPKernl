/// Trampoline for new processes
/// Expects: R4 = entry point, R5 = argument
/// Jumps to entry(arg)
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn process_trampoline() -> ! {
    core::arch::naked_asm!(
        "mov r0, r5",     // arg -> r0 (first parameter)
        "bx r4",          // jump to entry point in r4
    );
}

/// Switch context between two processes
/// r0 = pointer to old SP storage
/// r1 = new SP value
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn switch_context(old_sp_ptr: *mut *mut u32, new_sp: *const u32) {
    core::arch::naked_asm!(
        // Check if in interrupt state
        "mov r2, lr",

        // Shift r2 24 bits to get to top 
        "lsrs r2, r2, #24",
        // r3 0 means no interrupt, 1 means interrupted
        "movs r3, #1",
        "cmp r2, #0xFF",
        "beq interrupt_sp",

        // If not interrupt 
        // set r3 to be 0
        "movs r3, #0",

        // Save callee-saved registers and LR
        "push {{lr}}",

        // Interrupt lable, skipping lr 
        "interrupt_sp:",
        "push {{r4-r7}}",
        "mov r4, r8",
        "mov r5, r9",
        "mov r6, r10",
        "mov r7, r11",
        "push {{r4-r7}}",

        // Save current SP to old_sp_ptr
        "mov r2, sp",
        "str r2, [r0]",

        // Load new SP
        "mov sp, r1",

        // Restore R8-R11
        "pop {{r4-r7}}",
        "mov r8, r4",
        "mov r9, r5",
        "mov r10, r6",
        "mov r11, r7",

        // Restore R4-R7
        "pop {{r4-r7}}",

        // Exit via interrupt 
        "cmp r3, #1",
        "beq interrupt_exit",

        // Restore LR and return
        "pop {{pc}}",

        // Exit interrupt 
        "interrupt_exit:",
        "bx lr",
    );
}

/// Start the first process (never returns)
/// r0 = SP of first process
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn start_first_process(sp: *const u32) -> ! {
    core::arch::naked_asm!(
        // Set stack pointer
        "mov sp, r0",

        // Restore R8-R11
        "pop {{r4-r7}}",
        "mov r8, r4",
        "mov r9, r5",
        "mov r10, r6",
        "mov r11, r7",

        // Restore R4-R7 (R4=entry, R5=arg)
        "pop {{r4-r7}}",

        // Pop LR and jump to it (trampoline)
        "pop {{pc}}",
    );
}
