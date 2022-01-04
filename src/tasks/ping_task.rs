use crate::Task;
use crate::Gate;
use crate::debug::*;

pub struct PingTask { 
    gate: Gate,
}

impl Task<PingTask> for PingTask {
    fn new() -> PingTask {
        return PingTask {
            gate: Gate::new()
                .when_nano(crate::MS_TO_NANO * 3500, || {
                    debug_u64(crate::math::rand(), b"random");
                    debug_str(b"ping pong");
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