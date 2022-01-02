/*
This is the actual main entrypoint for custom
firmware. It is incorporated into the kernel
but exists on its own.
*/

pub mod clock_task;
pub mod ping_task;
pub mod ws2812_task;

use crate::Task;
use crate::tasks::clock_task::*;
use crate::tasks::ping_task::*;
use crate::tasks::ws2812_task::*;

pub struct Program {
    clock_task: ClockTask,
    ping_task: PingTask,
    ws2812_task: WS2812Task,
}

impl Program {
    pub fn new() -> Program {
        return Program {
            clock_task: ClockTask::new(),
            ping_task: PingTask::new(),
            ws2812_task: WS2812Task::new(),
        };
    }
}

impl Task for Program {
    fn init(&mut self) {
        self.clock_task.init();
        self.ping_task.init();
        self.ws2812_task.init();
    }

    fn system_loop(&mut self) {
        self.clock_task.system_loop();
        self.ping_task.system_loop();
        self.ws2812_task.system_loop(); 
    }
}