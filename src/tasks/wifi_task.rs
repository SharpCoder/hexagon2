use crate::Task;
use crate::debug::*;
use crate::drivers::wifi::*;
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
        self.driver.dns_lookup(b"worldtimeapi.org", &|outputs: Vector<Vector<u8>>| {
            // For this function, the first argument is the string
            // containing the ip address.
            
            debug_str(b"ZOMG\r\n");
            debug_u32(outputs.size() as u32, b"output size");
            blink_hardware(2);

            if outputs.size() > 0 {
                let mut value = outputs.clone().pop().unwrap();
                debug_u32(value.size() as u32, b"value size");
                while value.size() > 0 {
                    match value.dequeue() {
                        None => {},
                        Some(byte) => {
                            serial_write(SerioDevice::Uart4, &[byte]);
                        }
                    }
                }
                debug_str(b"Here");
            }
        });
    }

    fn system_loop(&mut self) {
        self.driver.process();
    }
}