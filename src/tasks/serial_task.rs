use crate::Task;
use crate::Gate;
use crate::debug::*;
use crate::serio::*;
use crate::datastructures::*;

pub struct SerialTask { 
    gate: Gate,
}

impl Task<SerialTask> for SerialTask {
    fn new() -> SerialTask {
        return SerialTask {
            gate: Gate::new()
                // .when_nano(crate::MS_TO_NANO * 1000, noop)
                .when(|gate: &mut Gate| {
                    return serio_available() > 0;
                }, || {

                    let mut buf = Buffer::<20, u8>::new(0u8);
                    while serio_available() > 0 {
                        match serio_read() {
                            None => {},
                            Some(word) => {
                                buf.push(word);
                            }
                        }
                    }


                    serio_write(b"received ");
                    serio_write(&buf.as_array());

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

fn noop() {}