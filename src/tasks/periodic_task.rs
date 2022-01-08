/*
This task manages the LED_13 pin which is used
exclusively by debug blinks. So it works in conjunction
with system debug to properly gate the LED based on
blink requests without tying up system resources.
*/

use crate::Task;
use crate::Gate;
use crate::debug::blink_hardware;

pub struct PeriodicTask { 
    gate: Gate,
}

impl PeriodicTask {
    pub fn new() -> PeriodicTask {
        return PeriodicTask {
            gate: Gate::new()
                .when_nano(crate::S_TO_NANO * 2, || {
                    // serial_write(SerioDevice::Default, b"AT\r\n");
                    // blink_hardware(1);
                })
                .compile()
        }
    }
}

impl Task for PeriodicTask {
    fn init(&mut self) { }
    fn system_loop(&mut self) {
        self.gate.process();
    }
}
