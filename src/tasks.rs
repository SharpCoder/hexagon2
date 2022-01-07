/*
This is the actual main entrypoint for custom
firmware. It is incorporated into the kernel
but exists on its own.
*/

use core::arch::asm;

pub mod wifi_task;
pub mod ws2812_task;
pub mod blink_task;
pub mod periodic_task;

use crate::Task;
use crate::drivers::wifi::*;
use crate::serio::*;
use crate::tasks::ws2812_task::*;
use crate::tasks::blink_task::*;
use crate::tasks::wifi_task::*;
use crate::tasks::periodic_task::*;

pub fn run_tasks() {
    // Drivers and stateful things
    let mut wifi_driver = WifiDriver::new(SerioDevice::Uart6, 5, 6);

    let mut rgb_task = WS2812Task::new();
    let mut blink_task = BlinkTask::new();
    let mut wifi_task = WifiTask::new(&mut wifi_driver);
    let mut periodic_task = PeriodicTask::new();

    rgb_task.init();
    blink_task.init();
    wifi_task.init();
    periodic_task.init();

    loop {
        periodic_task.system_loop();
        rgb_task.system_loop();
        blink_task.system_loop();
        wifi_task.system_loop();

        unsafe {
            asm!("nop");
        }
    }
    
}