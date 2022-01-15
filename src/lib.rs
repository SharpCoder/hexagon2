#![feature(lang_items, fn_traits)]
#![crate_type = "staticlib"]
#![no_std]
pub mod drivers;
pub mod http_models;
pub mod ws2812;
pub mod blink_task;
pub mod thermal_task;

use core::arch::asm;
use drivers::max31820::Max31820Driver;
use teensycore::*;
use teensycore::debug::*;
use teensycore::phys::pins::*;
use drivers::wifi::*;
use ws2812::*;
use blink_task::*;
use thermal_task::*;

teensycore::main!({
    // Drivers and stateful things
    let _wifi_driver = WifiDriver::new(SerioDevice::Default, 5, 6);
    let temp_driver = Max31820Driver::new(10);

    // Tasks
    let mut led_task = WS2812Task::new();
    let mut blink_task = BlinkTask::new();
    let mut thermal_task = ThermalTask::new(temp_driver);

    led_task.init();
    blink_task.init();
    thermal_task.init();
    
    loop {
        led_task.system_loop();
        blink_task.system_loop();
        thermal_task.system_loop();

        unsafe {
            asm!("nop");
        }
    }
});