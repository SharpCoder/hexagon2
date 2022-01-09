use crate::*;
use crate::Task;
use crate::drivers::wifi::*;
use crate::system::vector::*;
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
        // self.driver.init();

        debug_str(b"Resetting ESP8266");
        self.driver.reset();
        debug_str(b"Connecting to Wifi");
        self.driver.connect(b"Bird of Prey", b"password");
        self.driver.dns_lookup(b"worldtimeapi.org", &|driver, outputs: Vector<Vector<u8>>| {
            debug_str(b"DNS lookup complete");

            // For this function, the first argument is the string
            // containing the ip address.
            if outputs.size() > 0 {
                let ip_address = outputs.get(0).unwrap();
                driver.http_request(ip_address, HttpRequest {
                    method: vec_str!(b"GET"),
                    request_uri: vec_str!(b"/api/timezone/America/Los_Angeles.txt"),
                    host: vec_str!(b"worldtimeapi.org"),
                    headers: None,
                    content: None,
                }, &|_driver, _outputs| {
                    debug_str(b"HTTP Request complete");

                    let content = _outputs.get(0).unwrap();
                    debug_u32(content.size() as u32, b"Received size");
                    serial_write_vec(SerioDevice::Debug, content);
                });
            }
        });
    }

    fn system_loop(&mut self) {
        self.driver.process();
    }
}