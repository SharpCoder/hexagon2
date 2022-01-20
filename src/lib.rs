#![feature(lang_items, fn_traits)]
#![crate_type = "staticlib"]
#![no_std]
pub mod drivers;
pub mod ws2812;
pub mod blink_task;
pub mod thermal_task;
pub mod wifi_task;
pub mod http;

use core::arch::asm;
use drivers::max31820::Max31820Driver;
use teensycore::*;
use teensycore::phys::pins::*;
use ws2812::*;
use blink_task::*;
use thermal_task::*;
use teensycore::serio::*;
use wifi_task::*;

teensycore::main!({
    use teensycore::gate::*;

    // Drivers and stateful things
    // let _wifi_driver = WifiDriver::new(SerioDevice::Default, 5, 6);
    // let temp_driver = Max31820Driver::new(10);

    // Tasks
    let mut led_task = WS2812Task::new();
    let mut blink_task = BlinkTask::new();
    let mut wifi_task = WifiTask::new();

    // let mut thermal_task = ThermalTask::new(temp_driver);

    led_task.init();
    blink_task.init();
    wifi_task.init();
    // thermal_task.init();

    serial_init(SerioDevice::Default);

    loop {
        disable_interrupts();
        led_task.system_loop();
        enable_interrupts();

        blink_task.system_loop();
        wifi_task.system_loop();

        // disable_interrupts();
        // thermal_task.system_loop();
        // enable_interrupts();

        unsafe {
            asm!("nop");
        }
    }
});