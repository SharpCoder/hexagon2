use crate::*;
use crate::Task;
use crate::drivers::wifi::*;
use crate::system::vector::*;
use crate::system::strings::*;
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

pub static mut HTTP_COMPLETE: bool = false;

impl <'a> Task for WifiTask<'a> {
    fn init(&mut self) {
        debug_str(b"Resetting ESP8266");
        self.driver.reset();
        debug_str(b"Reconfiguring baud rate");
        // Set baud using both bauds in case it's stuck
        debug_str(b"Connecting to Wifi");
        self.driver.connect(b"Bird of Prey", b"password", &|_,_| {
            debug_str(b"Wifi Connected!");
        });
        self.driver.dns_lookup(b"worldtimeapi.org", &|driver, outputs: BTreeMap<String, String>| {
            debug_str(b"DNS lookup complete");

            // For this function, the first argument is the string
            // containing the ip address.
            if outputs.size() > 0 {
                let ip_address = outputs.get(vec_str!(b"ip_address")).unwrap();
                driver.http_request(ip_address, HttpRequest {
                    method: vec_str!(b"GET"),
                    request_uri: vec_str!(b"/api/timezone/America/Los_Angeles.txt"),
                    host: vec_str!(b"worldtimeapi.org"),
                    headers: None,
                    content: None,
                }, &|_driver, artifacts| {
                    debug_str(b"HTTP Request complete");
                    serial_write_vec(SerioDevice::Debug, artifacts.get(vec_str!(b"content")).unwrap());
                    unsafe {
                        HTTP_COMPLETE = true;
                    }
                });
            }
        });
    }

    fn handle_message(&mut self, _topic: String, _content: String) {
        
    }
    
    fn system_loop(&mut self) {
        self.driver.process();
    }
}