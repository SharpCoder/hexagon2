use teensycore::*;
use teensycore::clock::nanos;
use teensycore::debug::debug_str;
use teensycore::math::itoa;
use teensycore::serio::*;
use teensycore::system::str::*;
use teensycore::{debug::debug_u64, gate_open, S_TO_NANO};

use crate::drivers::msgeq7::MSGEQ7Driver;
pub struct AudioTask {
    driver: MSGEQ7Driver,
    next_event: u64,
}

impl AudioTask {
    pub fn new() -> Self {
        return AudioTask {
            driver: MSGEQ7Driver::new(
                9,
                11,
                10,
                12, 
            ),
            next_event: 0,
        }
    }

    pub fn init(&self) {

    }

    pub fn system_loop(&mut self) {
        
        if nanos() > self.next_event {
            let bands = self.driver.read();
            for i in 0 .. 7 {
                
                let mut msg = itoa(bands[i]);
                msg.join_with_drop(&mut str!(b" band "));
                msg.join_with_drop(&mut itoa(i as u64));
                msg.join_with_drop(&mut str!(b"\n"));

                serial_write_str(SerioDevice::Debug, &msg);

                msg.drop();
            }

            serial_write(SerioDevice::Debug, b"\n");
            self.next_event = S_TO_NANO + nanos();
        }
    }
}