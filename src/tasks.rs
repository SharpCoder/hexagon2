/*
This is the actual main entrypoint for custom
firmware. It is incorporated into the kernel
but exists on its own.
*/

use core::arch::asm;

pub mod clock_task;
pub mod ping_task;
pub mod ws2812_task;
pub mod blink_task;

use crate::Task;
use crate::list::*;
use crate::debug::*;
use crate::tasks::clock_task::*;
use crate::tasks::ping_task::*;
use crate::tasks::ws2812_task::*;
use crate::tasks::blink_task::*;

pub struct Program {
    clock_task: ClockTask,
    ping_task: PingTask,
    ws2812_task: WS2812Task,
}


pub fn run_tasks() {
    let mut stack = Stack::<u32>::new();
    stack.push(100);
    // stack.push(200);
    // stack.push(300);

    let mut task1 = ClockTask::new();
    let mut task2 = PingTask::new();
    let mut task3 = WS2812Task::new();
    let mut blink_task = BlinkTask::new();

    task1.init();
    task2.init();
    task3.init();
    blink_task.init();

    match stack.pop() {
        Some(number) => {
            debug_u32(number, b"pop");
        } ,
        None => {
            debug_str(b"no items");
        }
    }
    
    while true {
        task1.system_loop();
        task2.system_loop();
        task3.system_loop();
        blink_task.system_loop();
        unsafe {
            asm!("nop");
        }
    }
    
}