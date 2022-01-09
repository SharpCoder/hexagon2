/*
This task manages the LED_13 pin which is used
exclusively by debug blinks. So it works in conjunction
with system debug to properly gate the LED based on
blink requests without tying up system resources.
*/

use crate::Task;
use crate::Gate;
use crate::debug::*;

pub struct PeriodicTask { 
    gate: Gate,
}

pub struct OmgCats {
    pub lolz: u128,
    pub catz: u128,
}

static mut COUNT: u32 = 0;

impl PeriodicTask {
    pub fn new() -> PeriodicTask {
        return PeriodicTask {
            gate: Gate::new()
                .when_nano(crate::MS_TO_NANO * 50, || {                    
                    if crate::mem::is_overrun(core::mem::size_of::<OmgCats>()) {
                        debug_str(b"reclaimed");
                    } else {
                        debug_str(b"alloc");
                    }

                    let ptr = crate::mem::kalloc::<OmgCats>();
                    crate::mem::free(ptr);
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
