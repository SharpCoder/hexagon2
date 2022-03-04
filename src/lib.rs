#![feature(lang_items, fn_traits)]
#![crate_type = "staticlib"]
#![no_std]

pub mod drivers;
pub mod models;
pub mod shaders;
pub mod pixel_engine;
pub mod pixel_task;

// This is ugly but necessary
#[cfg(not(feature = "testing"))]
pub mod blink_task;
#[cfg(not(feature = "testing"))]
pub mod thermal_task;
#[cfg(not(feature = "testing"))]
pub mod wifi_task;
#[cfg(not(feature = "testing"))]
pub mod http;

use core::arch::asm;
use models::SystemCommand;
use teensycore::*;
use teensycore::phys::pins::*;
use teensycore::system::observable::Observable;
use pixel_task::*;

#[cfg(not(feature = "testing"))]
use {
    blink_task::*,
    wifi_task::*,
    thermal_task::*,
};

use teensycore::serio::*;
use teensycore::system::str::*;

// Create a system observable for process event
// handling.
static mut OBSERVER: Option<Observable<SystemCommand>> = None;
static mut OBSERVER_KEY: Option<Str> = None;

#[cfg(not(feature = "testing"))]
teensycore::main!({
    use teensycore::gate::*;
    use crate::*;

    // Drivers and stateful things
    // let _wifi_driver = WifiDriver::new(SerioDevice::Default, 5, 6);
    let thermal_driver = crate::drivers::max31820::Max31820Driver::new(10);

    // Setup event loop
    unsafe {
        OBSERVER = Some(Observable::new());
        OBSERVER_KEY = Some(str!(b"process_loop"));
    }

    // Tasks
    // let led_task = WS2812Task::get_instance();
    let mut blink_task = BlinkTask::new();
    // let mut wifi_task = WifiTask::new();
    // let mut audio_task = AudioTask::new();
    let mut thermal_task = ThermalTask::new(thermal_driver);
    let mut pixel_task = PixelTask::new();

    // led_task.init();
    blink_task.init();
    pixel_task.init();
    thermal_task.init();

    // wifi_task.init();
    serial_init(SerioDevice::Default);

    loop {
        disable_interrupts();
        // led_task.system_loop();
        pixel_task.system_loop();
        enable_interrupts();

        blink_task.system_loop();
        thermal_task.system_loop();
        // audio_task.system_loop();
        // wifi_task.system_loop();

        // If the thermal task has completed, we can transition
        // the loading indicator forward
        if thermal_task.loaded {
            pixel_task.ready();
        }

        unsafe {
            asm!("nop");
        }
    }
});


#[no_mangle]
#[inline]
pub fn test_wait(nanos: u64) {
    let cycles = (nanos - 55) / 10;
    for _ in 0 .. cycles {
        assembly!("nop");
    }
}

pub fn proc_handle(func: &'static dyn Fn(&SystemCommand)) {
    unsafe {
        OBSERVER.as_mut().unwrap().on(OBSERVER_KEY.as_ref().unwrap(), func);
    }
}

pub fn proc_emit(command: &SystemCommand) {
    unsafe {
        OBSERVER.as_mut().unwrap().emit(OBSERVER_KEY.as_ref().unwrap(), command);
    }
}