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


macro_rules! procs {
    ( $( $x:expr ),*, ) => {{
        $($x.init();)*
        loop {
            unsafe {
                $($x.system_loop();)*
                asm!("nop");
            }
        }
    }};
}

#[no_mangle]
pub fn run_tasks() {
    // Drivers and stateful things
    let mut wifi_driver = WifiDriver::new(SerioDevice::Uart6, 5, 6);
    crate::debug::blink(125, crate::debug::Speed::Fast);

    // The processes which run in this system
    let mut wifi_task = WifiTask::new(&mut wifi_driver);
    let mut blink_task = BlinkTask::new();
    let mut periodic_task = PeriodicTask::new();
    let mut led_task = WS2812Task::new();

    procs!(
        wifi_task,
        blink_task,
        periodic_task,
        led_task,
    );
}