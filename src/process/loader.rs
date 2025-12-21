use crate::process::*;
use crate::layout::MemoryLayout;

use core::result::Result;
use core::result::Result::{Ok, Err};


static mut NEXT_FREE: usize = 0; 
static mut ID: u8 = 0; 

#[derive(Debug)]
pub enum ProcessError {
    NoMemory, 
    InvalidSize, 
} 

/*
 * Return the start of the stack, given size
 * */
fn allocate_stack(size: usize) -> Result<*mut u8, ProcessError> {
    if size == 0 {
        return Err(ProcessError::InvalidSize);
    }
    let region = MemoryLayout::new(); 
    unsafe {
        if NEXT_FREE + size > region.processes.size {
            return Err(ProcessError::NoMemory);
        }
        else {
            let ptr: *mut u8 = (NEXT_FREE + region.processes.start) as *mut u8;
            NEXT_FREE += size; 
            return Ok(ptr); 
        }
    }
}

unsafe fn setup_initial_stack(stack_base: *mut u8, 
    stack_size: usize, entry: fn(*mut ()) -> !, arg: *mut()) -> *mut u32 {
    let mut sp = (stack_base as usize + stack_size) as *mut u32; 

    // Move down one and write some value
    unsafe {
        // Move down one, 
        sp = sp.offset(-1); 
        *sp = 0x0100_0000; // xPSR

        sp = sp.offset(-1);
        *sp = entry as u32; // PC

        sp = sp.offset(-1);
        *sp = 0;            // LR
                            
        // R12, R3, R2, R1
        for _ in 0..4 {
            sp = sp.offset(-1);
            *sp = 0;        
        }

        // R0 
        sp = sp.offset(-1); 
        *sp = arg as u32;
        
        // R4-R11
        for _ in 4..=11 {
            sp = sp.offset(-1);
            *sp = 0;        
        }
    }

    sp
}

pub unsafe fn create_process(entry: fn(* mut()) -> !, 
    stack_size: usize, arg: *mut ()) -> Result<PCB, ProcessError> {
    let stack_start = allocate_stack(stack_size)?;

    unsafe {
        let sp = setup_initial_stack(stack_start, stack_size, entry, arg);
        let id: u8 = ID; 
        ID += 1; 
        Ok(PCB {
            pid: id, 
            state: ProcessState::Ready, 

            sp: sp,
            stack_base: stack_start, 
            stack_size: stack_size, 
        })
    }
}
