#![feature(lang_items, fn_traits)]
#![crate_type = "staticlib"]
#![no_std]


#[cfg(feature = "testing")]
extern crate std;

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

use pixel_engine::shader_config::ShaderConfigList;
use teensycore::*;
use teensycore::clock::uNano;
use teensycore::clock::nanos;
use teensycore::phys::pins::*;
use pixel_task::*;

#[cfg(not(feature = "testing"))]
use {
    blink_task::*,
    wifi_task::*,
    thermal_task::*,
};

use teensycore::serio::*;
use teensycore::system::vector::Vector;

// Feature Flags
const USE_WIFI: bool = false;
const HEX_UNITS: usize = 4;
const CYCLE_MODE: bool = false;

// Time Constants
static MIN_IN_HOUR: uNano = 60;
static SECONDS_IN_MIN: uNano = 60;

// Random Stuff
static mut TRANSITION_DELAY_NANOS: uNano = 1 /* Hours */ * MIN_IN_HOUR * SECONDS_IN_MIN * teensycore::S_TO_NANO;
static mut WORLD_TIME_S: uNano = 0;
static mut UTC_OFFSET: uNano = 8;
static mut UPTIME_WORLDTIME_OFFSET_S: uNano = 0;
static mut SHADER_CONFIGS: ShaderConfigList = ShaderConfigList { configs: Vector { head: None, size: 0 } };

#[cfg(not(feature = "testing"))]
teensycore::main!({
    use crate::*;

    // Drivers and stateful things
    let thermal_driver = crate::drivers::max31820::Max31820Driver::new(10);

    // Tasks
    let mut blink_task = BlinkTask::new();
    let mut wifi_task = WifiTask::new();
    let mut thermal_task = ThermalTask::new(thermal_driver);
    let mut pixel_task = PixelTask::new();

    thermal_task.init();
    blink_task.init();
    pixel_task.init();

    if USE_WIFI {
        wifi_task.init();
    }

    serial_init(SerioDevice::Default);

    loop {
        pixel_task.system_loop();
        blink_task.system_loop();
        thermal_task.system_loop();

        if USE_WIFI {
            wifi_task.system_loop();
        }

        // If the thermal task has completed, we can transition
        // the loading indicator forward
        if USE_WIFI && wifi_task.ready {
            pixel_task.ready();
        } else if !USE_WIFI && thermal_task.loaded {
            pixel_task.ready();
        }
    }
});

/// Sets the current unix epoch in seconds
pub fn set_world_time(time_s: uNano) {
    unsafe {
        UPTIME_WORLDTIME_OFFSET_S = nanos() / S_TO_NANO;
        WORLD_TIME_S = time_s;
    }
}

/// Returns the current unix epoch relative to seconds
pub fn get_world_time() -> uNano {
    return unsafe {
        WORLD_TIME_S + (nanos() / S_TO_NANO) - UPTIME_WORLDTIME_OFFSET_S
    };
}

pub fn set_utc_offset(offset: uNano) {
    unsafe {
        UTC_OFFSET = offset;
    }
}

pub fn get_utc_offset() -> uNano {
    return unsafe {
        UTC_OFFSET
    };
}

pub fn get_tranasition_delay() -> uNano {
    return unsafe {
        TRANSITION_DELAY_NANOS
    };
}

pub fn set_transition_delay(nanos: uNano) {
    unsafe {
        TRANSITION_DELAY_NANOS = nanos;
    }
}

pub fn set_shader_configs(config_list: ShaderConfigList) {
    unsafe {
        SHADER_CONFIGS = ShaderConfigList {
            configs: config_list.configs.clone(),
        }
    };
}

pub fn get_shader_configs() -> &'static ShaderConfigList {
    return unsafe { &SHADER_CONFIGS };
}