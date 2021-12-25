#![feature(lang_items)]
#![crate_type = "staticlib"]
#![no_std]

pub mod phys;
pub mod drivers;

// Import assembly macro
use core::arch::asm;

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
    gpio_speed(Pin::Gpio7, MuxSpeed::Fast);
    gpio_direction(Pin::Gpio7, phys::Dir::Output);

    loop { 
        gpio_set(Pin::Gpio7, 0x1 << 3);

        let mut i = 0;
        while i < 20000000 {
            i = i + 1;
            unsafe {
                asm!("nop");
            }
    
        }

        i = 0;

        gpio_clear(Pin::Gpio7, 0x1 << 3);
        
        while i < 20000000 {
            i = i + 1;
            unsafe {
                asm!("nop");
            }
    
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