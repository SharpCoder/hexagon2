use crate::*;
use crate::clock;
use crate::phys::irq::{disable_interrupts, enable_interrupts};
use crate::serio::*;
use crate::phys::pins::*;
use crate::strings::*;
use crate::datastructures::*;

pub struct WifiDriver {
    device: SerioDevice,
    en_pin: usize,
    reset_pin: usize,
    queued_commands: Vector<WifiCommandSequence<'static>>,
    active_command: usize,
    time_target: u64,
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
        };
    }

    pub fn connect(&mut self, ssid: &[u8], pwd: &[u8]) {
        // Generate the command sequence
        self.queued_commands.enqueue( WifiCommandSequence::new(
            Vector::from_slice(&[
                WifiCommand::new().with_command(b"AT").with_expected_response(b"OK"),
                WifiCommand::new().with_command(b"AT+CWMODE=1").with_expected_response(b"OK"),
                WifiCommand::new().with_command(b"AT+CWJAP=\"")
                    .join_vec(vec_str!(ssid))
                    .join_vec(vec_str!(b"\",\""))
                    .join_vec(vec_str!(pwd))
                    .join_vec(vec_str!(b"\""))
                    .with_expected_response(b"OK"),
            ])
        ));
    }

    pub fn dns_lookup<'b>(&mut self, domain: &[u8], method: &'static Fn(Vector<Vector<u8>>)) {
        self.queued_commands.enqueue( WifiCommandSequence::new_with_callback(
            Vector::from_slice(&[
                WifiCommand::new().with_command(b"AT").with_expected_response(b"OK"),
                WifiCommand::new().with_command(b"AT+CIPDOMAIN=\"")
                    .join_vec(vec_str!(domain))
                    .join_vec(vec_str!(b"\""))
                    .with_expected_response(b"OK")
                    .with_transform(|buffer| {
                        // Holy smokes this is awful.
                        // If I'm going to do this more than... once... it needs
                        // some serios macro game or something.

                        // Really I should have a standardized way to parse the responses
                        // because they are pretty consistent I think.

                        // Probably should just make a string class at this point.
                        let start = index_of(buffer, &vec_str!(b":")).unwrap_or(0);
                        let end = index_of(buffer, &vec_str!(b"OK")).unwrap_or(0);
                        
                        let mut rx_buffer = (&buffer).substr(start + 1, end - start).unwrap();
                        let space = match index_of(&rx_buffer, &vec_str!(b"\r")) {
                            None => 0,
                            Some(val) => val,
                        };

                        return rx_buffer.substr(0, space).unwrap().clone();
                    })
                    
            ]),
            Box::new(method)
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
        if clock::nanos() < self.time_target {
            return;
        }
        self.time_target = clock::nanos() + crate::MS_TO_NANO * 50;

        let device = self.device;
        let driver = self;
        match driver.queued_commands.get_mut(driver.active_command) {
            None => { },
            Some(command) => {
                if command.is_complete() {
                    driver.active_command += 1;
                } else {
                    command.process(device, &Vector::from_slice(serial_buffer(device).clone()));
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

    /// If this is present, the receive buffer is the input
    /// and whatever you return goes into a register
    /// that is stored at the WifiCommandSequence level
    pub transform_output: Option<fn(&Vector::<u8>) -> Vector::<u8>>,
}

impl WifiCommand {

    pub fn new() -> Self {
        return WifiCommand {
            command: Vector::new(),
            expected_response: None,
            error_response: None,
            delay: 0,
            transform_output: None,
        };
    }

    pub fn with_transform(&self, transform_method: fn(&Vector::<u8>) -> Vector::<u8>) -> Self {
        let mut next = self.clone();
        next.transform_output = Some(transform_method);
        return next;
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

type Func = dyn Fn(Vector<Vector<u8>>);

#[derive(Copy, Clone)]
pub struct WifiCommandSequence<'a> {
    commands: Vector<WifiCommand>,
    outputs: Vector<Vector<u8>>,
    index: usize,
    command_sent: bool,
    time_target: u64,
    complete: bool,
    aborted: bool,
    callback: Option<Box<&'a Func>>,
}

/// A WifiCommandSequence is a list of commands
/// to process in order. Each command will only
/// advance to the next one after a command criteria
/// has been met.
impl <'a> WifiCommandSequence<'a> {
    pub fn new(commands: Vector<WifiCommand>) -> WifiCommandSequence<'a> {
        return WifiCommandSequence {
            commands: commands,
            outputs: Vector::new(),
            command_sent: false,
            index: 0,
            time_target: 0,
            complete: false,
            aborted: false,
            callback: None,
        };
    }

    pub fn new_with_callback(commands: Vector<WifiCommand>, func: Box<&'a Func>) -> WifiCommandSequence<'a> {
        return WifiCommandSequence {
            commands: commands,
            outputs: Vector::new(),
            command_sent: false,
            index: 0,
            time_target: 0,
            complete: false,
            aborted: false,
            callback: Some(func),
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
                        // Check if there is transformation to happen
                        if command.transform_output.is_some() {
                            let transform_method = command.transform_output.unwrap();
                            self.outputs.push(transform_method(&Vector::from_slice(serial_buffer(device) )));
                        }
                        
                        self.advance(device);
                    }
                } else if serial_available(device) > 0 {
                    // Scan for the things we care about
                    match command.expected_response {
                        None => {},
                        Some(expected_response) => {
                            if contains(rx_buffer, &expected_response) {

                                // Check if there is transformation to happen
                                if command.transform_output.is_some() {
                                    let transform_method = command.transform_output.unwrap();
                                    self.outputs.push(transform_method(rx_buffer));
                                }

                                self.advance(device);
                            }
                        }
                    }
        
                    match command.error_response {
                        None => {},
                        Some(error_response) => {
                            if contains(rx_buffer, &error_response) {
                                self.reset(device);
                                self.aborted = true;
                            }
                        }
                    }
                }

                crate::irq::enable_interrupts();
            }
        }
        
        self.update_time_target();
    }

    fn advance(&mut self, device: SerioDevice) {
        self.index += 1;
        if self.index >= self.commands.size() {
            if self.callback.is_some() {
                let method = self.callback.unwrap().reference;
                (*method)(self.outputs);
            }
            self.complete = true;
            // Reset state
            self.reset(device);
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