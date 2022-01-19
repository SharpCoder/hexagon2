use teensycore::*;
use teensycore::clock::*;
use teensycore::debug::*;

use crate::drivers::max31820::Max31820Driver;

/// This task is responsible for sampling the ambiant temperature
/// and keeping the system informed of changes.

pub struct ThermalTask {
    driver: Max31820Driver,
    next_event: u64,
}

impl ThermalTask {
    pub fn new(driver: Max31820Driver) -> Self {
        return ThermalTask {
            driver: driver,
            next_event: 0,
        };
    }

    pub fn init(&self) {
    
    }

    pub fn system_loop(&mut self) {
        if nanos() > self.next_event {
            
            // New line
            debug_str(b"");

            self.driver.test();
            
            // match self.driver.read_rom() {
            //     None => {
            //         debug_str(b"failed");
            //     },
            //     Some(temp) => {
            //         // Set temperature somewhere.
            //         // debug_u64(temp, b"rom code");
            //     }
            // }

            self.next_event = nanos() + S_TO_NANO * 5;
        }
    }
}