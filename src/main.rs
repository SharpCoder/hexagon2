#![feature(lang_items, fn_traits)]
#![crate_type = "staticlib"]
#![no_std]
pub mod clock;
pub mod debug;
pub mod drivers;
pub mod gate;
pub mod http_models;
pub mod math;
pub mod mem;
pub mod phys;
pub mod serio;
pub mod tasks;
pub mod system;

// NOTE: remove this if you have trouble building
pub mod env;

use core::arch::asm;
use core::arch::global_asm;
use phys::*;
use phys::irq::*;
use phys::pins::*;
use debug::*;
use serio::*;
use gate::*;

pub const S_TO_NANO: u64 = 1000000000;
pub const MS_TO_NANO: u64 = S_TO_NANO / 1000;

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
    serial_init(SerioDevice::Default);
    serial_init(SerioDevice::Debug);

    // Enable interrupts across the system
    enable_interrupts();

    debug_str(b"begin memtest...");
    mem::memtest();
    debug_str(b"memtest complete");

    // Run system tasks
    // Go see tasks.rs to add more system tasks
    // to the kernel.
    tasks::run_tasks();
    
    loop {
        unsafe {
            asm!("nop");
        }
    }
}

#[derive(Copy, Clone)]
pub struct Box<T : Clone + Copy> {
    reference: T,
}

impl <T : Clone + Copy> Box<T> {
    pub fn new(reference: T) -> Self {
        return Box { reference: reference };
    }
}


pub trait Task {
    fn init(&mut self);
    fn system_loop(&mut self);
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


pub enum PanicType {
    Hardfault,
    Memfault,
    Oob,
}

#[no_mangle]
pub fn err(mode: PanicType) {
    disable_interrupts();
    loop {
        match mode {
            PanicType::Hardfault => {
                pin_out(13, Power::High);
            },
            PanicType::Oob => {
                pin_out(13, Power::High);
                wait_ns(MS_TO_NANO * 50);
                pin_out(13, Power::Low);
                wait_ns(MS_TO_NANO * 500);
            },
            PanicType::Memfault => {
                pin_out(13, Power::High);
                wait_ns(MS_TO_NANO * 1500);
                pin_out(13, Power::Low);
                wait_ns(MS_TO_NANO * 50);
            }
        }
    }
}

#[no_mangle]
pub fn oob() {
    err(PanicType::Oob);
}

extern "C" {
    pub fn uuid() -> u32;
}

// Although I'm including the core library, I like
// my own implementation better.
global_asm!("
    _ZN4core9panicking18panic_bounds_check17h9048f255eeb8dcc3E:
        bl oob
        b hang

    hang:
        b hang

    uuid:
        mov r0, lr
        mov pc, lr
");