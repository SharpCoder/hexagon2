use crate::Task;
use crate::Gate;
use crate::debug::*;

pub struct ClockTask { 
    gate: Gate,
}

impl Task<ClockTask> for ClockTask {
    fn new() -> ClockTask {
        return ClockTask {
            gate: Gate::new()
                .when_nano(crate::MS_TO_NANO * 1000, || {
                    debug_u64(crate::clock::nanos() / crate::MS_TO_NANO, b"beep boop");
                    blink(2, Speed::Fast);
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