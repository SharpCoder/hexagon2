#![feature(lang_items, fn_traits)]
#![crate_type = "staticlib"]
#![no_std]

pub mod drivers;
pub mod ws2812;
pub mod models;

// This is ugly but necessary
#[cfg(not(feature = "testing"))]
pub mod blink_task;
#[cfg(not(feature = "testing"))]
pub mod thermal_task;
#[cfg(not(feature = "testing"))]
pub mod wifi_task;
#[cfg(not(feature = "testing"))]
pub mod http;
#[cfg(not(feature = "testing"))]
pub mod audio_task;

use core::arch::asm;
use models::SystemCommand;
use teensycore::*;
use teensycore::phys::pins::*;
use teensycore::system::observable::Observable;
use ws2812::*;


#[cfg(not(feature = "testing"))]
use {
    blink_task::*,
    wifi_task::*,
    thermal_task::*,
    audio_task::*,
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
    // let temp_driver = Max31820Driver::new(10);

    // Setup event loop
    unsafe {
        OBSERVER = Some(Observable::new());
        OBSERVER_KEY = Some(str!(b"process_loop"));
    }

    // Tasks
    let led_task = WS2812Task::get_instance();
    let mut blink_task = BlinkTask::new();
    let mut wifi_task = WifiTask::new();
    let mut audio_task = AudioTask::new();

    // let mut thermal_task = ThermalTask::new(temp_driver);

    led_task.init();
    blink_task.init();
    audio_task.init();

    // wifi_task.init();
    // thermal_task.init();

    serial_init(SerioDevice::Default);

    loop {
        disable_interrupts();
        led_task.system_loop();
        enable_interrupts();

        blink_task.system_loop();
        audio_task.system_loop();
        // wifi_task.system_loop();

        // disable_interrupts();
        // thermal_task.system_loop();
        // enable_interrupts();

    
        unsafe {
            asm!("nop");
        }
    }
});

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