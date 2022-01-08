use crate::*;
use crate::Task;
use crate::drivers::wifi::*;
use crate::datastructures::*;
use crate::http_models::*;

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
        self.driver.dns_lookup(b"worldtimeapi.org", &|driver, outputs: Vector<Vector<u8>>| {
            // For this function, the first argument is the string
            // containing the ip address.
            if outputs.size() > 0 {
                let ip_address = outputs.clone().pop().unwrap();
                driver.http_request(ip_address, HttpRequest {
                    method: vec_str!(b"GET"),
                    request_uri: vec_str!(b"/api/timezone/America/Los_Angeles.txt"),
                    host: vec_str!(b"worldtimeapi.org"),
                    headers: None,
                    content: None,
                }, &|_driver, _outputs| {

                });
            }
        });
    }

    fn system_loop(&mut self) {
        self.driver.process();
    }
}