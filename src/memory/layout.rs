use core::clone::Clone;
use core::marker::Copy;

pub struct MemoryRegion {
    pub start: usize, 
    pub size: usize, 
}

impl MemoryRegion {
    // Return the end address of the memory region
    pub fn end(&self) -> usize {
        self.start + self.size 
    }

    // Within memory region 
    pub fn contains(&self, addr: usize) -> bool {
        addr >= self.start && addr < self.end()
    }
}

impl Clone for MemoryRegion {
    fn clone(&self) -> Self {
        MemoryRegion {
            start: self.start,
            size: self.size,
        }
    }
}

impl Copy for MemoryRegion {}

pub struct MemoryLayout {
    pub kernel_data: MemoryRegion, 
    pub wifi: MemoryRegion, 
    pub processes: MemoryRegion, 
}

impl MemoryLayout {
    pub fn new() -> Self {
        unsafe extern "C" {
            // Kernel
            static _kernel_data_start: u8;
            static _kernel_data_size: usize;

            // Wifi 
            static _wifi_start: u8; 
            static _wifi_size: usize;

            // Processes 
            static _processes_start: u8;
            static _processes_size: usize; 
        }


        unsafe {
            let kernel_data = MemoryRegion{ 
                start: &_kernel_data_start as *const u8 as usize, 
                size: *(&_kernel_data_size as *const usize),
            }; 

            let wifi = MemoryRegion{
                start: &_wifi_start as *const u8 as usize,  
                size: *(&_wifi_size as *const usize),
            };

            let processes = MemoryRegion {
                start: &_processes_start as *const u8 as usize,  
                size: *(&_processes_size as *const usize),
            }; 

            Self {
                kernel_data: kernel_data,
                wifi: wifi, 
                processes: processes,
            }
        }
    }
}
