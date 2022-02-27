#![feature(lang_items, fn_traits)]
#![crate_type = "staticlib"]
#![no_std]

pub mod drivers;
pub mod ws2812_task;
pub mod models;
pub mod shaders;

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
use ws2812_task::*;


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

    let mut change = 0;
    let mut iteration = 0;

    serial_init(SerioDevice::Default);

    loop {
        disable_interrupts();
        led_task.system_loop();

        // One shot operation
        // if change == 0 {
        //     one_shot();
        //     change = 1;
        // }

        enable_interrupts();

        blink_task.system_loop();
        // audio_task.system_loop();
        // wifi_task.system_loop();


        // disable_interrupts();
        // thermal_task.system_loop();
        // enable_interrupts();

    
        unsafe {
            asm!("nop");
        }
    }
});

#[inline]
#[no_mangle]
pub fn one_shot() {

    let mut error = 0;
    // Test conditions
    for cond in 1 ..= 10 {
        let wait = cond * 150;
        let start = nanos();
        test_wait(wait);
        test_wait(wait);
        let end = nanos();

        error += (end - start) - 296 - wait;
        debug::debug_u64(wait * 2, b"wait");
        debug::debug_u64((end - start) - 296, b"time to read nanos\n");
    }

    debug::debug_u64(error, b"total error");
}

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