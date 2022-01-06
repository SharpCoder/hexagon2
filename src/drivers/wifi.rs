use crate::clock;
use crate::gate::*;
use crate::debug::blink_hardware;
use crate::phys::irq::{disable_interrupts, enable_interrupts};
use crate::serio::*;
use crate::phys::pins::*;
use crate::strings::*;
use crate::datastructures::*;

pub struct WifiDriver {
    device: SerioDevice,
    en_pin: usize,
    reset_pin: usize,
}

impl WifiDriver {
    pub const fn new(device: SerioDevice, en_pin: usize, reset_pin: usize) -> Self {
        return WifiDriver {
            device: device,
            en_pin: en_pin,
            reset_pin: reset_pin,
        };
    }

    pub fn init(&self) {
        // Enable peripheral
        pin_mode(self.reset_pin, Mode::Output);
        pin_mode(self.en_pin, Mode::Input);

        disable_interrupts();
        pin_out(self.reset_pin, Power::Low);
        crate::wait_ns(crate::MS_TO_NANO * 50);
        pin_out(self.reset_pin, Power::High);
        crate::wait_ns(crate::MS_TO_NANO * 800);
        enable_interrupts();
    }

    fn emit(&self, msg: &[u8]) {
        serial_write(self.device, msg);
        serial_write(self.device, b"\r\n");
    }
}

#[derive(Copy, Clone)]
pub struct WifiCommand {
    pub command: &'static [u8],
    pub expected_response: Option<&'static [u8]>,
    pub error_response: Option<&'static [u8]>,
}

impl WifiCommand {
    pub const fn new(command: &'static [u8]) -> WifiCommand {
        return WifiCommand {
            command: command,
            expected_response: None,
            error_response: None,
        }
    }

    pub fn new_with_response(command: &'static [u8], expected_response: &'static [u8]) -> WifiCommand {
        return WifiCommand {
            command: command,
            expected_response: Some(expected_response),
            error_response: None,
        };
    }
}

pub struct WifiCommandSequence <'a> {
    commands: Vector<WifiCommand>,
    driver: &'a WifiDriver,
    index: usize,
    command_sent: bool,
    time_target: u64,
    complete: bool,
}

impl <'a> WifiCommandSequence <'a> {
    pub fn new(driver: &'a WifiDriver, commands: Vector<WifiCommand>) -> WifiCommandSequence<'a> {
        return WifiCommandSequence {
            driver: driver,
            commands: commands,
            command_sent: false,
            index: 0,
            time_target: 0,
            complete: false,
        };
    }

    pub fn process(&mut self) {
        if self.complete || clock::nanos() < self.time_target {
            return;
        }

        let driver = self.driver;
        match self.commands.get(self.index) {
            None => {
                self.advance();
            },
            Some(command) => {

                if !self.command_sent {
                    self.driver.emit(command.command);
                    self.command_sent = true;
        
                    // Check if we care about the response
                    if command.expected_response.is_none() && command.error_response.is_none() {
                        self.advance();
                    }
                } else if serial_available(driver.device) > 0 {
                    // Scan for the things we care about
                    match command.expected_response {
                        None => {},
                        Some(expected_response) => {
                            if contains(serial_buffer(driver.device), expected_response) {
                                self.advance();
                            }
                        }
                    }
        
                    match command.error_response {
                        None => {},
                        Some(error_response) => {
                            if contains(serial_buffer(driver.device), error_response) {
                                crate::err();
                            }
                        }
                    }
                }
            }
        }
        
        self.update_time_target();
    }

    fn advance(&mut self) {
        self.index += 1;
        if self.index >= self.commands.size() {
            self.complete = true;
        } else {
            // Reset state
            self.reset();
        }
    }

    fn reset(&mut self) {
        self.command_sent = false;
    }

    fn update_time_target(&mut self) {
        self.time_target = clock::nanos() + crate::MS_TO_NANO * 100;
    }
}