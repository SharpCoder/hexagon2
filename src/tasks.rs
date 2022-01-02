/*
This is the actual main entrypoint for custom
firmware. It is incorporated into the kernel
but exists on its own.
*/

pub mod clock_task;
pub mod ping_task;

use crate::Task;
use crate::tasks::clock_task::*;
use crate::tasks::ping_task::*;

pub struct Program {
    clock_task: ClockTask,
    ping_task: PingTask,
}

impl Program {
    pub fn new() -> Program {
        return Program {
            clock_task: ClockTask::new(),
            ping_task: PingTask::new(),
        };
    }
}

impl Task for Program {
    fn init(&mut self) {
        self.clock_task.init();
        self.ping_task.init();
    }

    fn system_loop(&mut self) {
        self.clock_task.system_loop();
        self.ping_task.system_loop();
    }
}