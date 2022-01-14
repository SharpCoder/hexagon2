#![feature(lang_items, fn_traits)]
#![crate_type = "staticlib"]
#![no_std]
pub mod drivers;
pub mod http_models;
pub mod ws2812;

use core::arch::asm;
use teensycore::phys::pins::*;
use drivers::wifi::*;
use ws2812::*;

teensycore::main!({

    // Drivers and stateful things
    let mut wifi_driver = WifiDriver::new(SerioDevice::Default, 5, 6);
    let mut led_task = WS2812Task::new();
    
    // teensycore::debug::debug_str(b"hellO");
    led_task.init();

    loop {
        led_task.system_loop();
    
        unsafe {
            asm!("nop");
        }
    }

});