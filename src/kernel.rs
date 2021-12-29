#![feature(lang_items, generators)]
#![crate_type = "staticlib"]
#![no_std]
pub mod phys;
pub mod drivers;
pub mod clock;
pub mod serio;
pub mod debug;

use core::arch::asm;
use core::arch::global_asm;
use phys::irq::*;
use phys::uart::Baud;
use serio::*;
use phys::*;
use phys::gpio::{ 
    gpio_speed,
    gpio_direction,
    gpio_set,
    gpio_clear,
    MuxSpeed,
    Pin,
};


#[no_mangle]
#[link_section = "testdata"]
static test_data: [u32; 8] = [
    125,
    325,
    525,
    625,
    725,
    825,
    925,
    1125,
];

static test_data_2: u32 = 1052;

#[no_mangle]
pub fn main() {
    // Initialize irq system, (disables all interrupts)
    irq_init();
    disable_interrupts();

    // Initialize clocks
    phys_clocks_en();

    // Initialize serial communication
    serio_init();
    serio_baud(Baud::Rate9600);

    // Setup GPIO pin 13 (the LED on teensy)
    gpio_speed(Pin::Gpio7, MuxSpeed::Fast);
    gpio_direction(Pin::Gpio7, phys::Dir::Output);

    // Ignite system clock for keeping track of millis()
    // which is also used for the wait implementation.
    clock::clock_init();

    // Enable interrupts across the system
    enable_interrupts();

    loop { 
        unsafe {

            // gpio_set(Pin::Gpio7, 0x1 << 3);
            // drivers::ws2812::ws2812_loop();
            // gpio_set(Pin::Gpio7, 0x1 << 3);
            // wait_ns(100000000); // 100000000
            // gpio_clear(Pin::Gpio7, 0x1 << 3);
            // wait_ns(100000000); // 100000000
            // if clock::nanos() > 0 {
            //     gpio_set(Pin::Gpio7, 0x1 << 3);
            // } else {
            //     gpio_clear(Pin::Gpio7, 0x1 << 3);
            // }

            debug::blink(1, debug::Speed::Normal);
            serio_write_byte(b'a');
            asm!("nop");
        }
        
    }
}

pub fn wait_wow(_nano: u64) {
    let mut r = 0;
    while r < 50000000 {
        r = r + 1;
        unsafe { asm!( "nop"); }
    }
}

pub fn wait_ns(nano: u64) {
    unsafe {
        let origin = clock::nanos();
        while (origin + nano) > clock::nanos() {
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

// Yeah how tf do you do this in rust? Idk
// These functions take a pointer and return the absolute address
// to that pointer as a word. Somehow.
global_asm!("
    ptr_to_addr_byte:
        add r0, sp, #4

    ptr_to_addr_word:
        add r0, sp, #4
");