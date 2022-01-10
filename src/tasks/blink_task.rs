/*
This task manages the LED_13 pin which is used
exclusively by debug blinks. So it works in conjunction
with system debug to properly gate the LED based on
blink requests without tying up system resources.
*/

use crate::*;
use crate::Task;
use crate::Gate;
use crate::system::strings::*;

static mut NEXT_BLINK_EVENT: u64 = 0x0;

pub struct BlinkTask { }
impl BlinkTask {
    pub fn new() -> BlinkTask {
        return BlinkTask { };
    }
}

impl Task for BlinkTask {
    fn init(&mut self) { }
    
    fn system_loop(&mut self) {
        gate_open!()
            .when(|_| unsafe { BLINK_CONFIG.remaining_count } > 0, || {
                unsafe {
                    NEXT_BLINK_EVENT = clock::nanos() + BLINK_CONFIG.speed as u64;
                }
            })
            .when(|_| clock::nanos() > unsafe { NEXT_BLINK_EVENT }, || {
                unsafe {
                    if BLINK_CONFIG.remaining_count % 2 == 0 {
                        blink_led_on();
                    } else {
                        blink_led_off();
                    }

                    BLINK_CONFIG.remaining_count -= 1;
                }
            })
            .compile();
    }

    fn handle_message(&mut self, _topic: String, _content: String) {
        
    }
}
