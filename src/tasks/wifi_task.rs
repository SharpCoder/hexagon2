use crate::Task;
use crate::drivers::wifi::*;

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
        self.driver.dns_lookup(b"worldtimeapi.org", |res| {

        });
    }

    fn system_loop(&mut self) {
        self.driver.process();
    }
}