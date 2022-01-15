#![feature(lang_items, fn_traits)]
#![crate_type = "staticlib"]
#![no_std]
pub mod drivers;
pub mod http_models;
pub mod ws2812;
pub mod blink_task;

use core::arch::asm;
use teensycore::*;
use teensycore::debug::*;
use teensycore::phys::pins::*;
use drivers::wifi::*;
use ws2812::*;
use blink_task::*;

teensycore::main!({

    // Seed random number generator
    let seed = clock::nanos();
    teensycore::math::seed_rand(seed);

    // Get a random number
    let rand = teensycore::math::rand();
    debug_u64(rand, b"random");

    // Drivers and stateful things
    let _wifi_driver = WifiDriver::new(SerioDevice::Default, 5, 6);
    let mut led_task = WS2812Task::new();
    let mut blink_task = BlinkTask::new();
    
    led_task.init();
    blink_task.init();

    loop {
        led_task.system_loop();
        blink_task.system_loop();
        teensycore::debug::blink_accumulate();

        unsafe {
            asm!("nop");
        }
    }
});