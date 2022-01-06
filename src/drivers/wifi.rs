use crate::debug::blink_hardware;
use crate::phys::irq::{disable_interrupts, enable_interrupts};
use crate::serio::*;
use crate::phys::pins::*;

pub struct WifiDriver {
    device: SerioDevice,
    en_pin: usize,
    reset_pin: usize,
}

impl WifiDriver {
    pub fn new(device: SerioDevice, en_pin: usize, reset_pin: usize) -> Self {
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
        crate::wait_ns(crate::MS_TO_NANO * 50);
        enable_interrupts();

        for i in 0 .. 3 {
            self.emit(b"AT");
            crate::wait_ns(crate::MS_TO_NANO * 1000);
        }
    }

    fn emit(&self, msg: &[u8]) {
        serial_write(self.device, msg);
        serial_write(self.device, b"\r\n");
    }
}