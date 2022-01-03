use crate::Task;
use crate::Gate;
use crate::debug::*;

pub struct PingTask { 
    gate: Gate,
}

impl Task for PingTask {
    fn init(&mut self) {
    }

    fn system_loop(&mut self) {
        self.gate.process();
    }
}

impl PingTask {
    pub fn new() -> PingTask {
        return PingTask {
            gate: Gate::new()
                .when_nano(1000000000, || {
                    debug_str(b"ping pong");
                })
                .compile()
        }
    }
}