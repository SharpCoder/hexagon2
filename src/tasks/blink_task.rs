/*
This task manages the LED_13 pin which is used
exclusively by debug blinks. So it works in conjunction
with system debug to properly gate the LED based on
blink requests without tying up system resources.
*/

use crate::Task;
use crate::Gate;
use crate::debug;

pub struct BlinkTask { 
    gate: Gate,
}

static mut NEXT_BLINK_EVENT: u64 = 0x0;

impl Task<BlinkTask> for BlinkTask {
    fn new() -> BlinkTask {
        return BlinkTask {
            gate: Gate::new()
                .when(|_this: &mut Gate| {
                    return unsafe { debug::BLINK_CONFIG.remaining_count } > 0;
                }, || {
                    unsafe {
                        NEXT_BLINK_EVENT =
                            crate::clock::nanos() + 
                            debug::BLINK_CONFIG.speed as u64;
                    }
                })
                .when(|_this: &mut Gate| {
                    return crate::clock::nanos() > unsafe { NEXT_BLINK_EVENT };
                }, || {
                    if unsafe { debug::BLINK_CONFIG.remaining_count } % 2 == 0 {
                        debug::blink_led_on();
                    } else {
                        debug::blink_led_off();
                    }

                    unsafe {
                        debug::BLINK_CONFIG.remaining_count -= 1;
                    }
                })
                .compile()
        }
    }

    fn init(&mut self) {
    }

    fn system_loop(&mut self) {
        self.gate.process();
    }
}
