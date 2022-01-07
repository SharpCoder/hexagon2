use crate::Task;
use crate::Gate;
use crate::drivers::wifi;
use crate::drivers::wifi::*;
use crate::debug::*;
use crate::serio::*;
use crate::datastructures::*;

pub struct WifiTask<'a> { 
    // startup_sequence: WifiCommandSequence,
    driver: &'a mut WifiDriver,
}

impl <'a> WifiTask<'a> {
    pub fn new(wifi_driver: &'a mut WifiDriver) -> WifiTask<'a> {
        return WifiTask {
            driver: wifi_driver,
        }
    }
}

impl <'a> Task for WifiTask<'a> {
    fn init(&mut self) {
        self.driver.init();
        self.driver.connect(b"Bird of Prey", b"password");
    }

    fn system_loop(&mut self) {
        self.driver.process();
    }
}

fn noop() {}