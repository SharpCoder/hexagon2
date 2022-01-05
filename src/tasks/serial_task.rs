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
                .when_nano(crate::MS_TO_NANO * 850, noop)
                .when(|gate: &mut Gate| {
                    return serio_available() > 5;
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
                    

                    let arr = buf.data;
                    let comp = b"Hello";
                    let mut matched = false;

                    for i in 0 .. 20 {
                        if comp[0] == arr[i] {

                            // Compare the rest of the word
                            matched = true;
                            for r in 0 .. comp.len() {
                                if arr[i + r] != comp[r] {
                                    matched = false;
                                    break;
                                }
                            }

                            if matched {
                                break;
                            }
                        }
                    }

                    if matched {
                        blink(2, Speed::Fast);
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

fn noop() {}