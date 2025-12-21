use crate::process::*;

/*
Save old process context
1. Push R4-R11 onto current stack
2. Save current SP into old_pcb.sp
3. Load new_pcb.sp into SP register
4. Pop R4-R11 from new stack
5. Return (hardware will restore R0-R3, R12, LR, PC, PSR)

r0 correspond to old_pcb 
r1 correspond to new_pcb
*/
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn switch_context(old_pcb: *mut PCB, new_pcb: *const PCB) {
    core::arch::naked_asm!(
        // 1.
        "push {{r4-r7}}",          // Save callee-saved registers
        // ARM Cortex-M0+ only let push and pop on r0-r7
        "mov r4, r8",
        "mov r5, r9", 
        "mov r6, r10", 
        "mov r7, r11", 
        "push {{r4-r7}}",

        // 2. 
        "mov r2, sp",               // Get stack pointer 
        "str r2, [r0, #8]",         // Store the r2 (current sp) to old_pcb->sp

        // 3. 
        "ldr r2, [r1, #8]",         // Load sp from new_pcb to r2 
        "mov sp, r2",               // Place new_pcb->sp to current sp 
                                    
        // 4. 
        "pop {{r4-r7}}",           // Pop from callee

        // Put R4 content, which is R8 old value back into R8 and pop it
        "mov r8, r4", 
        "mov r9, r5",
        "mov r10, r6",
        "mov r11, r7",
        "pop {{r4-r7}}",

        // 5. 
        "bx lr",                    // Return 
    );
}

