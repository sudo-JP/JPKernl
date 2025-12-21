use crate::process::PCB;
use crate::layout::MemoryLayout;

use core::result::Result;
use core::result::Result::{Ok, Err};

fn allocate_stack(size: usize) {

}

pub enum ProcessError {
    NoMem
} 

pub fn create_process(entry: fn() -> !, 
    stack_size: usize) -> Result<PCB, ProcessError> {

    Err(ProcessError::NoMem)
}
