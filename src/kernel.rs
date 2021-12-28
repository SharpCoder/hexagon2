#![feature(lang_items)]
#![crate_type = "staticlib"]
#![no_std]
pub mod phys;
pub mod drivers;
pub mod clock;

// Import assembly macro
use core::arch::asm;
use phys::irq::*;

use phys::gpio::{ 
    gpio_speed,
    gpio_direction,
    gpio_set,
    gpio_clear,
    MuxSpeed,
    Pin,
};


#[no_mangle]
pub fn main() {
    // Initialize irq syste, (disables all interrupts)
    irq_init();
    enable_interrupts();

    gpio_speed(Pin::Gpio7, MuxSpeed::Fast);
    gpio_direction(Pin::Gpio7, phys::Dir::Output);

    clock::clock_init();

    loop { 
        unsafe {
            // gpio_set(Pin::Gpio7, 0x1 << 3);
            // drivers::ws2812::ws2812_loop();
            gpio_set(Pin::Gpio7, 0x1 << 3);
            wait_ns(100000000); // 100000000
            gpio_clear(Pin::Gpio7, 0x1 << 3);
            wait_ns(100000000); // 100000000
            // if clock::nanos() > 0 {
            //     gpio_set(Pin::Gpio7, 0x1 << 3);
            // } else {
            //     gpio_clear(Pin::Gpio7, 0x1 << 3);
            // }
            // debug_blink(2);
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

pub fn debug_blink(count: u32) {
    let mut i = 0;
    while i < count {
        gpio_set(Pin::Gpio7, 0x1 << 3);
        wait_wow(1);
        gpio_clear(Pin::Gpio7, 0x1 << 3);
        wait_wow(1);
        i += 1;
    }
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}#[panic_handler]

#[no_mangle]
pub extern fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop { }
}