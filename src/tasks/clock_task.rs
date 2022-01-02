use crate::Task;
use crate::Gate;
use crate::kernel::debug::*;

pub struct ClockTask { 
    gate: Gate,
}

impl Task for ClockTask {
    fn init(&mut self) {
    }

    fn system_loop(&mut self) {
        self.gate.process();
    }
}

impl ClockTask {
    pub fn new() -> ClockTask {
        return ClockTask {
            gate: Gate::new()
                .when_nano(100000000, || {
                    debug_u64(crate::clock::nanos(), b"beep boop");
                    blink(2, Speed::Fast);
                })
                .compile()
        }
    }
}