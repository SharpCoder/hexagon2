#![feature(lang_items, fn_traits)]
#![crate_type = "staticlib"]
#![no_std]
pub mod drivers;
// pub mod http_models;
pub mod ws2812;
pub mod blink_task;
pub mod thermal_task;

use core::arch::asm;
use drivers::max31820::Max31820Driver;
use teensycore::*;
use teensycore::phys::pins::*;
// use drivers::wifi::*;
use ws2812::*;
use blink_task::*;
use thermal_task::*;
use teensycore::serio::*;

teensycore::main!({
    use teensycore::gate::*;

    // Drivers and stateful things
    // let _wifi_driver = WifiDriver::new(SerioDevice::Default, 5, 6);
    // let temp_driver = Max31820Driver::new(10);

    // Tasks
    let mut led_task = WS2812Task::new();
    let mut blink_task = BlinkTask::new();
    // let mut thermal_task = ThermalTask::new(temp_driver);

    led_task.init();
    blink_task.init();
    // thermal_task.init();

    serial_init(SerioDevice::Default);

    loop {
        disable_interrupts();
        led_task.system_loop();
        enable_interrupts();

        blink_task.system_loop();

        loop {
            if serial_available(SerioDevice::Default) > 0 {
                let sb = serial_read(SerioDevice::Default);
                // Consume the buffer.
                sb.clear();
            } else {
                break;
            }
        }

        gate_open!()
            .when_nano(S_TO_NANO, || {
                disable_interrupts();
                serial_write(SerioDevice::Debug, b"hello world, ping pong!");
                enable_interrupts();
            }).compile();
            

        // disable_interrupts();
        // thermal_task.system_loop();
        // enable_interrupts();

        unsafe {
            asm!("nop");
        }
    }
});