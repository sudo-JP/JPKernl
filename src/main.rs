#![no_std]
#![no_main]

use panic_halt as _;
use rp2040_hal as hal;
// Some traits we need
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use hal::pac;
use core::ptr;

use rp2040_scheduler::{create_process, yield_now, Scheduler, CURRENT, PROCS, SCHEDULER};

#[unsafe(link_section = ".boot2")]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;


/// External high-speed crystal on the Raspberry Pi Pico board is 12 MHz. Adjust
/// if your board has a different frequency
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

static mut COUNTER1: u32 = 0;
static mut COUNTER2: u32 = 0;

#[rp2040_hal::entry]
fn main() -> ! {
    /*
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .unwrap();

    let mut timer = rp2040_hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins to their default state
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Configure GPIO0 as an output

    let mut led_pin = pins.gpio0.into_push_pull_output(); */
    let stack_size = 1024; 
    unsafe {
        let pcb1 = match create_process(proc1, stack_size) {
            Ok(p) => p, 
            Err(_) => panic!("Unable to create process")
        };
        let pcb2 = match create_process(proc2, stack_size) {
            Ok(p) => p, 
            Err(_) => panic!("Unable to create process")
        };

        PROCS[pcb1.pid as usize] = Some(pcb1);
        PROCS[pcb2.pid as usize] = Some(pcb2);

        let sched = ptr::addr_of_mut!(SCHEDULER); 
        (*sched).enqueue(pcb1.pid).unwrap();
        (*sched).enqueue(pcb2.pid).unwrap();

        CURRENT = Some(pcb1.pid);
        
    }
    
    loop {
        // TODO: This will eventually be your scheduler loop
    }
}

fn proc1() -> ! {
    loop {
        unsafe { COUNTER1 += 1; yield_now(); }
    }
}

fn proc2() -> ! {
    loop {
        unsafe { COUNTER2 += 1; yield_now(); }
    }
}

/*fn blink_fast() -> ! {
    loop {
        led_pin.set_high().unwrap();
        timer.delay_ms(500);
        led_pin.set_low().unwrap();
        timer.delay_ms(500);
    }
}

fn blink_slow() -> ! {
    // Configure GPIO0 as an output
    let mut led_pin = pins.gpio0.into_push_pull_output();
    loop {
        led_pin.set_high().unwrap();
        timer.delay_ms(1000);
        led_pin.set_low().unwrap();
        timer.delay_ms(1000);
    }
}*/

