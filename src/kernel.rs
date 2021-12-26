#![feature(lang_items)]
#![crate_type = "staticlib"]
#![no_std]

pub mod phys;
pub mod drivers;

// Import assembly macro
use core::arch::asm;
use drivers::ws2812::{
    ws2812_init,
    ws2812_loop,
};

use phys::{
    assign,
    irq::{
        attach_irq,
        enable_irq,
        Irq,
    }
};

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

    attach_irq(Irq::GPT1, interrupt_handler);
    enable_irq(Irq::GPT1);

    gpio_speed(Pin::Gpio7, MuxSpeed::Fast);
    gpio_direction(Pin::Gpio7, phys::Dir::Output);

    // Enable timers
    // assign(0x4008_4000, 0x0);
    // assign(0x4008_4000 + 0x100, 0x500);

    // ws2812_init();
    let mut set = false;
    loop { 
        let mut r = 0;
        gpio_set(Pin::Gpio7, 0x1 << 3);
        while r < 20000000 {
            r = r + 1;
            unsafe { asm!("nop"); }
        }
        r = 0;
        gpio_clear(Pin::Gpio7, 0x1 << 3);
        while r < 20000000 {
            r = r + 1;
            unsafe { asm!("nop"); }
        }
        // ws2812_loop();
        unsafe {
            asm!("nop");
            if set == false {
                enable_timer();
                set = true;
            }
        }
    }
}

pub fn enable_timer() {
    unsafe {
        // Disable GPT
        assign(0x401E_C000, 0x0);
        // Clear the IR
        assign(0x401E_C000 + 0xC, 0x0);
        // Clear Status Registers
        assign(0x401E_C000 + 0x8, 0x1F);
        // Enable interrupts
        assign(0x401E_C000 + 0xC, 0x11);
        // Select clock source & enable
        assign(0x401E_C000, 0x41);
        // Set compare value
        assign(0x401E_C000 + 0x10, 0x04FF_1000);

    }
}

pub fn pendsv() {
    unsafe {
        *((0xE000ED04) as *mut u32) = 0x10000000;
    }
}

#[no_mangle]
pub fn interrupt_handler() {
    loop {
        // Assign GPIO7 to pin 13 high
        let mut r = 0;
        gpio_set(Pin::Gpio7, 0x1 << 3);
        while r < 100000000 {
            r = r + 1;
            unsafe { asm!("nop"); }
        }
        r = 0;
        gpio_clear(Pin::Gpio7, 0x1 << 3);
        while r < 100000000 {
            r = r + 1;
            unsafe { asm!("nop"); }
        }

        unsafe {
            asm!("nop");
        }
    }
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}#[panic_handler]

#[no_mangle]
pub extern fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop { }
}