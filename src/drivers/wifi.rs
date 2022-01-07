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
    queued_commands: Vector<WifiCommandSequence>,
    active_command: usize,
    time_target: u64,
    rx_buffer: Vector<u8>,
}

impl WifiDriver {
    pub fn new(device: SerioDevice, en_pin: usize, reset_pin: usize) -> Self {
        return WifiDriver {
            device: device,
            en_pin: en_pin,
            reset_pin: reset_pin,
            queued_commands: Vector::new(),
            active_command: 0,
            time_target: 0,
            rx_buffer: Vector::new(),
        };
    }

    pub fn connect(&mut self, ssid: &[u8], pwd: &[u8]) {
        // Generate the command sequence
        self.queued_commands.enqueue( WifiCommandSequence::new(
            Vector::from_slice(&[
                WifiCommand::new().with_command(b"AT").with_expected_response(b"OK"),
                WifiCommand::new().with_command(b"AT+CWMODE=1").with_expected_response(b"OK"),
                WifiCommand::new().with_command(b"AT+CWJAP=\"")
                    .join_vec(Vector::from_slice(ssid))
                    .join_vec(Vector::from_slice(b"\",\""))
                    .join_vec(Vector::from_slice(pwd))
                    .join_vec(Vector::from_slice(b"\""))
                    .with_expected_response(b"OK"),
            ])
        ));
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

    pub fn emit(device: SerioDevice, msg: Vector::<u8>) {
        let mut message = msg.clone();
        message.join(Vector::from_slice(b"\r\n"));
        serial_write_vec(device, message);
    }

    pub fn process(&mut self) {
        // if clock::nanos() < self.time_target {
        //     return;
        // }
        // self.time_target = clock::nanos() + crate::MS_TO_NANO * 50;

        if serial_available(self.device) > 0 {
            match serial_read(self.device) {
                None => {},
                Some(byte) => {
                    // self.rx_buffer.enqueue(byte);
                    serial_write(SerioDevice::Uart4, &[byte]);
                }
            }
            crate::wait_ns(100000);
        }

        let device = self.device;
        let buffer = self.rx_buffer;
        let driver = self;
        match driver.queued_commands.get_mut(driver.active_command) {
            None => { },
            Some(command) => {
                if command.is_complete() {
                    driver.active_command += 1;
                } else {
                    command.process(device, &buffer);
                }
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct WifiCommand {
    pub command: Vector::<u8>,
    pub expected_response: Option<&'static [u8]>,
    pub error_response: Option<&'static [u8]>,
    pub delay: u64,
}

impl WifiCommand {

    pub fn new() -> Self {
        return WifiCommand {
            command: Vector::new(),
            expected_response: None,
            error_response: None,
            delay: 0,
        };
    }

    pub fn with_command(&self, command: &'static [u8]) -> Self {
        let mut next = self.clone();
        next.command.join( Vector::from_slice(command));
        return next;
    }

    pub fn join_vec(&self, vec_to_join: Vector::<u8>) -> Self {
        let mut next = self.clone();
        next.command.join(vec_to_join);
        return next;
    }

    pub fn with_expected_response(&self, response: &'static [u8]) -> Self {
        let mut next = self.clone();
        next.expected_response = Some(response);
        return next;
    }

    pub fn with_delay(&self, delay: u64) -> Self {
        let mut next = self.clone();
        next.delay = delay;
        return next;
    }
}

#[derive(Copy, Clone)]
pub struct WifiCommandSequence {
    commands: Vector<WifiCommand>,
    index: usize,
    command_sent: bool,
    time_target: u64,
    complete: bool,
    aborted: bool,
}

/// A WifiCommandSequence is a list of commands
/// to process in order. Each command will only
/// advance to the next one after a command criteria
/// has been met.
impl WifiCommandSequence {
    pub fn new(commands: Vector<WifiCommand>) -> WifiCommandSequence {
        return WifiCommandSequence {
            commands: commands,
            command_sent: false,
            index: 0,
            time_target: 0,
            complete: false,
            aborted: false,
        };
    }

    pub fn is_complete(&self) -> bool {
        return self.complete;
    }

    pub fn is_aborted(&self) -> bool {
        return self.aborted;
    }

    pub fn process(&mut self, device: SerioDevice, rx_buffer: &Vector<u8>) {
        if self.aborted || self.complete {
            return;
        }
        
        match self.commands.get(self.index) {
            None => {
                crate::err();
            },
            Some(command) => {
                if clock::nanos() < (self.time_target + command.delay) {
                    return;
                }

                crate::irq::disable_interrupts();

                if !self.command_sent {
                    WifiDriver::emit(device, command.command);
                    self.command_sent = true;
        
                    // Check if we care about the response
                    if command.expected_response.is_none() && command.error_response.is_none() {
                        self.advance(device);
                    }
                } else if serial_available(device) > 0 {
                    // Scan for the things we care about
                    match command.expected_response {
                        None => {},
                        Some(expected_response) => {
                            if contains(rx_buffer, &expected_response) {
                                crate::err();
                                self.advance(device);
                            }
                        }
                    }
        
                    // match command.error_response {
                    //     None => {},
                    //     Some(error_response) => {
                    //         if contains(serial_buffer(driver.device), error_response) {
                    //             self.aborted = true;
                    //         }
                    //     }
                    // }
                }

                crate::irq::enable_interrupts();
            }
        }
        
        self.update_time_target();
    }

    fn advance(&mut self, device: SerioDevice) {
        self.index += 1;
        crate::err();
        if self.index >= self.commands.size() {
            self.complete = true;
        } else {
            // Reset state
            self.reset(device);
        }
    }

    fn reset(&mut self, device: SerioDevice) {
        // Reset command lock
        self.command_sent = false;
        // Flush the serial buffer
        serial_clear_rx(device);
    }

    fn update_time_target(&mut self) {
        self.time_target = clock::nanos() + crate::MS_TO_NANO * 250;
    }
}