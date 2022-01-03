#![feature(lang_items)]
#![crate_type = "staticlib"]
#![no_std]
pub mod phys;
pub mod kernel;
pub mod drivers;
pub mod clock;
pub mod gate;
pub mod tasks;

use core::arch::asm;
use core::arch::global_asm;
use phys::*;
use phys::irq::*;
use phys::pins::*;
use kernel::debug::*;
use kernel::serio::*;
use kernel::list::*;
use gate::*;

pub const S_TO_NANO: u64 = 1000000000;
pub const MS_TO_NANO: u64 = 1000000;

#[no_mangle]
pub fn main() {
    // Initialize irq system, (disables all interrupts)
    disable_interrupts();

    // Initialize clocks
    phys_clocks_en();

    // Setup GPIO pin 13 (the LED on teensy)
    pin_mode(13, Mode::Output);

    // Ignite system clock for keeping track of millis()
    // which is also used for the wait implementation.
    clock::clock_init();

    // Setup serial
    serio_init();
    serio_baud(9600.0);

    // Enable interrupts across the system
    enable_interrupts();

    debug_str(b"======== New Instance ========");
    debug_hex(irq_addr(), b"IRQ Address");
    debug_u32(irq_size() as u32, b"IRQ Size");
    debug_str(b"");

    tasks::run_tasks();

    while true {
        unsafe {
            asm!("nop");
        }
    }
    
}

pub trait Task {
    fn init(&mut self) {}
    fn system_loop(&mut self) {}
}

pub fn wait(ms: u64) {
    return wait_ns(MS_TO_NANO * ms);
}

pub fn wait_ns(nano: u64) {
    unsafe {
        let origin = clock::nanos();
        let target = nano;
        while (origin + target) > clock::nanos() {
            asm!("nop");
        }
    }
}

pub fn pendsv() {
    unsafe {
        *((0xE000ED04) as *mut u32) = 0x10000000;
    }
}

pub fn dsb() {
    unsafe {
        asm!("dsb");
    }
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}#[panic_handler]

#[no_mangle]
pub extern fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop { }
}

#[no_mangle]
pub fn err() {
    pin_out(13, Power::High);
    loop {
        unsafe { asm!("nop"); }
    }
}

// Although I'm including the core library, I like
// my own implementation better.
global_asm!("
    _ZN4core9panicking18panic_bounds_check17h9048f255eeb8dcc3E:
        bl      err
        b hang

    hang:
        b hang
");