/// Debug module
/// Time_Task reads the millis() from an arduino and compares it
/// with our system clock. Used to determine wtf the actual
/// clock speed is.

use teensycore::serio::*;
use teensycore::clock::*;
use teensycore::system::vector::*;
use teensycore::system::strings::*;
use teensycore::math::*;
use teensycore::*;
use teensycore::debug::*;

pub struct TimeTask {
    line: String,
    offset: u64,
}

impl TimeTask {
    pub fn new() -> Self {
        return TimeTask {
            line: Vector::new(),
            offset: 0,
        }
    }

    fn process(&mut self) {
        // Check if line is throwaway
        match self.line.get(0) {
            None => {},
            Some(byte) => {
                if byte == b'~' {
                    self.line.clear();
                } else {
                    // This is a time!
                    let time = atoi_u64(self.line) * MS_TO_NANO;
                    if self.offset == 0 {
                        self.offset = time - nanos();
                    } else {
                        let sys_time = nanos() + self.offset;
                        let error = match time > sys_time {
                            true => time - sys_time,
                            false => sys_time - time,
                        };

                        serial_write(SerioDevice::Debug, b"~");
                        debug_u64(error / MS_TO_NANO, b"error");
                    }

                }
            }
        }

        self.line.clear();
    }

    pub fn init(&self) {
        serial_init(SerioDevice::Default);
    }

    pub fn system_loop(&mut self) {
        if serial_available(SerioDevice::Default) > 0 {
            match serial_read(SerioDevice::Default) {
                None => {},
                Some(byte) => {
                    if byte == b'\n' {
                        // Process line
                        self.process();
                    } else {
                        self.line.push(byte);
                    }
                }
            }
        }
    }
}