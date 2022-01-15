use teensycore::{clock::nanos, S_TO_NANO, debug::debug_u32};

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
        self.driver.initialize();
    }

    pub fn system_loop(&mut self) {
        if nanos() > self.next_event {
            match self.driver.read_temperature() {
                None => {},
                Some(temp) => {
                    // Set temperature somewhere.
                    debug_u32(temp as u32, b"[temp sample]");
                }
            }

            self.next_event = nanos() + S_TO_NANO * 10;
        }
    }
}