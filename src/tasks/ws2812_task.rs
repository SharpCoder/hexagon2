use crate::Task;
use crate::drivers::ws2812::*;
use crate::clock::*;

const LEDS: usize = 1;
  
pub struct WS2812Task { 
    target: u64,
    driver: WS2812Driver<LEDS>,
    color: u8,
}

impl Task for WS2812Task {
    fn init(&mut self) {
        self.driver.set_color(0, 0xFF0000);
    }

    fn system_loop(&mut self) {
        if nanos() > self.target {
            self.color += 1;
            if self.color > 255 {
                self.color = 0;
            }
            
            self.driver.set_color(0, wheel(self.color));
            self.target = nanos() + 10000000;
        }
    }
}

impl WS2812Task {
    pub fn new() -> WS2812Task {
        return WS2812Task { 
            color: 0,
            target: 0,
            driver: WS2812Driver::<LEDS>::new(
                18, // pin
            ),
        };
    }
}