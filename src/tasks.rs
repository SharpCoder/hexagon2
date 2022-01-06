/*
This is the actual main entrypoint for custom
firmware. It is incorporated into the kernel
but exists on its own.
*/

use core::arch::asm;

pub mod wifi_task;
pub mod ws2812_task;
pub mod blink_task;

use crate::Task;
use crate::debug::*;
use crate::tasks::ws2812_task::*;
use crate::tasks::blink_task::*;
use crate::tasks::wifi_task::*;

pub fn run_tasks() {
    let mut rgb_task = WS2812Task::new();
    let mut blink_task = BlinkTask::new();
    let mut wifi_task = WifiTask::new();

    rgb_task.init();
    blink_task.init();
    wifi_task.init();

    loop {
        // blink(1, Speed::Fast);

        rgb_task.system_loop();
        blink_task.system_loop();
        wifi_task.system_loop();

        unsafe {
            asm!("nop");
        }
    }
    
}