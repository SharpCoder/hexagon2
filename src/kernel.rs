#![feature(lang_items)]
#![crate_type = "staticlib"]
#![no_std]
pub mod phys;
// pub mod drivers;
pub mod math;
pub mod clock;
pub mod serio;
pub mod debug;
pub mod mem;

use core::panic::PanicInfo;
use core::arch::asm;
use core::arch::global_asm;
use phys::gpio::*;
use phys::irq::*;
use phys::uart::Baud;
use serio::*;
use phys::pins::*;
use phys::*;

const TX_PIN: usize = 1;
const RX_PIN: usize = 0;

#[no_mangle]
pub fn main() {
    // Initialize irq system, (disables all interrupts)
    disable_interrupts();
    // irq_init();

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
    // pendsv();
    debug::blink(4, debug::Speed::Fast);

    loop { 
        unsafe {
            // pin_out(TX_PIN, Power::High);
            // wait_ns(500000);
            // pin_out(TX_PIN, Power::Low);
            // wait_ns(500000);
            serio_write(b"Hello\n");
            // serio_write_byte(b'a');

            wait_wow(1);
            asm!("nop");
        }
        
    }
}

pub fn wait_wow(_nano: u64) {
    let mut r = 0;
    while r < 3500000 {
        r = r + 1;
        unsafe { asm!( "nop"); }
    }
}

pub fn wait_ns(nano: u64) {
    unsafe {
        let origin = clock::nanos();
        let target = nano / 10;
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

extern "C" {
    pub fn ptr_to_addr_word(ptr: *const u32) -> u32;
    pub fn ptr_to_addr_byte(ptr: *const u8) -> u32;
} 

#[no_mangle]
pub fn err() {
    pin_out(13, Power::High);
    loop {
        unsafe { asm!("nop"); }
    }
}

// Yeah how tf do you do this in rust? Idk
// These functions take a pointer and return the absolute address
// to that pointer as a word. Somehow.
global_asm!("
    ptr_to_addr_word:
    ptr_to_addr_byte:
        add r0, sp, #4
        mov pc, lr

    _ZN4core9panicking18panic_bounds_check17h9048f255eeb8dcc3E:
        bl      err
        b hang

    hang:
        b hang
");