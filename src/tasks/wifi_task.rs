use crate::Task;
use crate::Gate;
use crate::drivers::wifi::*;
use crate::debug::*;
use crate::serio::*;
use crate::datastructures::*;

static DRIVER: WifiDriver = WifiDriver::new(SerioDevice::Uart6, 5, 6);

pub struct WifiTask { 
    startup_sequence: WifiCommandSequence<'static>,
    driver: &'static WifiDriver,
}

impl WifiTask {
    pub fn new() -> WifiTask {
        return WifiTask {
            driver: &DRIVER,
            startup_sequence: WifiCommandSequence::new(
                &DRIVER,
                Vector::from_slice(&[
                    WifiCommand::new_with_response(b"AT", b"OK"),
                    WifiCommand::new_with_response(b"AT+CWMODE=1", b"OK"),
                    WifiCommand::new_with_response(b"AT+CWJAP=\"Bird of Prey\",\"password\"", b"OK"),
                    WifiCommand::new(b"AT+CIPDOMAIN=\"worldtimeapi.org\""),
                    WifiCommand::new_with_response(b"AT+CIPSTART=\"TCP\",\"213.188.196.246\",80", b"OK"),
                ])
            ),
        }
    }
}

impl Task for WifiTask {
    fn init(&mut self) {
        serio_init();
        self.driver.init();
    }

    fn system_loop(&mut self) {
        self.startup_sequence.process();
    }
}

fn noop() {}