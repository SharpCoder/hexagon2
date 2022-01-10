/*
This task manages the LED_13 pin which is used
exclusively by debug blinks. So it works in conjunction
with system debug to properly gate the LED based on
blink requests without tying up system resources.
*/

use crate::*;
use crate::Task;
use crate::debug::*;
use crate::phys::pins::*;
use crate::system::strings::*;

pub struct PeriodicTask { }

pub struct OmgCats {
    pub lolz: u128,
    pub catz: u128,
}

impl PeriodicTask {
    pub fn new() -> PeriodicTask {
        return PeriodicTask { };
    }
} 

impl Task for PeriodicTask {
    fn init(&mut self) { }
    fn system_loop(&mut self) {
        gate_open!()
            .when_nano(crate::MS_TO_NANO * 250, || { pin_out(13, Power::High )})
            .when_nano(crate::MS_TO_NANO * 250, || { pin_out(13, Power::Low); })
            .compile();

        if crate::mem::is_overrun() {
            debug_str(b"[erro] memory overflow");
        }

        gate_open!()
            .when_nano(crate::MS_TO_NANO * 1500, || { debug_str(b"hello world"); })
            .when_nano(crate::MS_TO_NANO * 400, || { debug_str(b"lolcatz"); })
            .compile();
    }

    fn handle_message(&mut self, _topic: String, _content: String) {
        
    }
}
