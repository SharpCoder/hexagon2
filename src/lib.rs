#![feature(lang_items, fn_traits)]
#![crate_type = "staticlib"]
#![no_std]

pub mod drivers;
pub mod models;
pub mod shaders;
pub mod effects;
pub mod pixel_engine;
pub mod pixel_task;
pub mod date_time;

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
static mut WORLD_TIME_S: u64 = 0;
static mut UTC_OFFSET: u64 = 8;
static mut UPTIME_WORLDTIME_OFFSET_S: u64 = 0;

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
    let mut blink_task = BlinkTask::new();
    let mut wifi_task = WifiTask::new();
    let mut thermal_task = ThermalTask::new(thermal_driver);
    let mut pixel_task = PixelTask::new();

    blink_task.init();
    pixel_task.init();
    thermal_task.init();
    wifi_task.init();

    serial_init(SerioDevice::Default);

    loop {
        pixel_task.system_loop();
        blink_task.system_loop();
        thermal_task.system_loop();
        wifi_task.system_loop();

        // If the thermal task has completed, we can transition
        // the loading indicator forward
        if thermal_task.loaded && wifi_task.ready {
            pixel_task.ready();
        }

        unsafe {
            asm!("nop");
        }
    }
});

/// Sets the current unix epoch in seconds
pub fn set_world_time(time_s: u64) {
    unsafe {
        UPTIME_WORLDTIME_OFFSET_S = nanos() / S_TO_NANO;
        WORLD_TIME_S = time_s;
    }
}

/// Returns the current unix epoch relative to seconds
pub fn get_world_time() -> u64 {
    return unsafe {
        WORLD_TIME_S + (nanos() / S_TO_NANO) - UPTIME_WORLDTIME_OFFSET_S
    };
}

pub fn set_utc_offset(offset: u64) {
    unsafe {
        UTC_OFFSET = offset;
    }
}

pub fn get_utc_offset() -> u64 {
    return unsafe {
        UTC_OFFSET
    };
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