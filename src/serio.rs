/** 
 * This module represents the serial communication protocol
 * based on UART physical hardware. For simplicity, it is tightly
 * coupled to a specific uart device.
*/

use core::arch::asm;
use crate::debug;
use crate::phys::addrs;
use crate::phys::uart::*;
use crate::phys::irq::*;
use crate::phys::dma::*;
use crate::phys::pins::*;
use crate::phys::xbar::*;
use crate::phys::*;

const UART_BUFFER_SIZE: usize = 512; // bytes
static mut UART_DEVICES: [Uart; 8] = [
    Uart::new(Device::Uart1, /* TX Pin */ 24, /* RX Pin */ 25, /* IRQ */ Irq::UART1),
    Uart::new(Device::Uart2, 14, 15, Irq::UART2),
    Uart::new(Device::Uart3, 17, 16, Irq::UART3),
    Uart::new(Device::Uart4, 8, 7, Irq::UART4),
    Uart::new(Device::Uart5, 0, 0, Irq::UART5), // NOTE: THIS DEVICE DOESN'T HAVE VALID PINS
    Uart::new(Device::Uart6, 1, 0, Irq::UART6),
    Uart::new(Device::Uart7, 29, 28, Irq::UART7),
    Uart::new(Device::Uart8, 20, 21, Irq::UART8),
];

#[derive(Clone, Copy)]
pub enum SerioDevice {
    Uart1 = 0x1,
    Uart2 = 0x2,
    Uart3 = 0x3,
    Uart4 = 0x4,
    Uart5 = 0x5,
    Uart6 = 0x6,
    Uart7 = 0x7,
    Uart8 = 0x8,
}

// The device serial communication is hardcoded tos
pub const SERIO_DEV: Device = Device::Uart6;
pub const SERIO_IRQ: Irq = Irq::UART6;
pub const TX_PIN: usize = 1;
pub const RX_PIN: usize = 0;

/** 
    This encapsulates an entire Uart device
    being instantiated, including all necessary memory
    and mappings.
*/
struct Uart {
    device: Device,
    tx_pin: usize,
    rx_pin: usize,
    initialized: bool,
    irq_processing: bool,
    irq: Irq,
    buffer: [u8; UART_BUFFER_SIZE],
    buffer_head: usize,
}

impl Uart {
    pub const fn new(device: Device, tx_pin: usize, rx_pin: usize, irq: Irq) -> Uart {
        return Uart {
            device: device,
            buffer: [0; UART_BUFFER_SIZE],
            buffer_head: 0,
            initialized: false,
            irq_processing: false,
            tx_pin: tx_pin,
            rx_pin: rx_pin,
            irq: irq,
        }
    }

    fn initialize(&mut self) {
        // Initialize the pins
        pin_mux_config(self.tx_pin, Alt::Alt2);
        pin_pad_config(self.tx_pin, PadConfig {
            hysterisis: true,
            resistance: PullUpDown::PullDown100k,
            pull_keep: PullKeep::Keeper,
            pull_keep_en: false,
            open_drain: false,
            speed: PinSpeed::Max200MHz,
            drive_strength: DriveStrength::MaxDiv3,
            fast_slew_rate: true,
        });

        uart_disable(&self.device);
        uart_sw_reset(&self.device, true);
        uart_sw_reset(&self.device, false);
        uart_configure(&self.device, UartConfig {
            r9t8: false,
            invert_transmission_polarity: false,
            overrun_irq_en: false,
            noise_error_irq_en: false,
            framing_error_irq_en: false,
            parity_error_irq_en: false,
            tx_irq_en: false,
            rx_irq_en: false,
            tx_complete_irq_en: true,
            idle_line_irq_en: false,
            tx_en: false,
            rx_en: false,
            match1_irq_en: false,
            match2_irq_en: false,
            idle_config: IdleConfiguration::Idle16Char,
            doze_en: false,
            bit_mode: BitMode::EightBits,
            parity_en: false,
            parity_type: ParityType::Even,
        });
    
        uart_configure_fifo(&self.device, FifoConfig {
            tx_fifo_underflow_flag: false,
            rx_fifo_underflow_flag: false,
            tx_flush: false,
            rx_flush: false,
            tx_fifo_overflow_irq_en: false,
            rx_fifo_underflow_irq_en: false,
            tx_fifo_en: true,
            tx_fifo_depth: BufferDepth::Data128Words,
            rx_fifo_en: false,
            rx_fifo_depth: BufferDepth::Data1Word,
        });
    
        uart_set_pin_config(&self.device, InputTrigger::Disabled);
        uart_disable_fifo(&self.device);
    
        attach_irq(&self.irq, uart_irq_handler);
        irq_enable(&self.irq);
    
        uart_watermark(&self.device);
        uart_enable(&self.device);
        
        // pin_mode(self.tx_pin, Mode::Output);
        // pin_out(self.tx_pin, Power::Low);
        
        // Default baud rate
        self.set_baud(9600.0);
        self.initialized = true;
    }

    pub fn set_baud(&self, rate: f32) {
        uart_baud_rate(&self.device, rate);
    }

    pub fn write(&mut self, bytes: &[u8]) {
        if !self.initialized {
            self.initialize();
        }

        let mut byte_idx = 0;
        while byte_idx < bytes.len() {
            self.enqueue(bytes[byte_idx]);
            byte_idx += 1;
        }
    }

    fn enqueue(&mut self, byte: u8) {
        self.buffer[self.buffer_head] = byte;
        self.buffer_head += 1;

        if self.buffer_head > self.buffer.len() {
            // This is technically a fatal condition I think.
            // TODO: error logging of some kind.
            self.buffer_head = 0;
        }
    }

    fn dequeue(&mut self) -> Option<u8> {
        // This would def be a no-hire if it were an interview :P
        if self.buffer_head > 0 {
            // Take the head element
            let result = self.buffer[0];

            // Shift everything over to the left
            let mut idx = 0;
            while idx < self.buffer_head {
                self.buffer[idx] = self.buffer[idx + 1];
                idx += 1;
            }

            // Decrement the head
            self.buffer_head -= 1;
            return Some(result);
        } else {
            return None;
        }
    }

    pub fn handle_irq(&mut self) {
        // Don't process a uart device that hasn't
        // been used
        if !self.initialized || self.irq_processing {
            return;
        }
        // This prevents circular calls
        self.irq_processing = true;
        irq_disable(&self.irq);

        // If tx is empty
        if uart_get_irq_statuses(&self.device) & (0x1 << 23) > 0 {
            match self.dequeue() {
                None => {
                    // Disengage, I guess?
                },
                Some(byte) => {
                    // Clear TSC
                    uart_disable(&self.device);
                    uart_enable(&self.device);
                    uart_sbk(&self.device);

                    // Get the next byte to write and beam it
                    uart_write_fifo(&self.device, byte);
                }
            }
        }

        uart_clear_irq(&self.device);
        irq_enable(&self.irq);
        self.irq_processing = false;
    }
}

// The main function to use unless you are an advanced user
pub fn serio_write(bytes: &[u8]) {
    serial_write(&SerioDevice::Uart6, bytes);
}

pub fn serio_baud(rate: f32) {
    let mut device = unsafe { &UART_DEVICES[SerioDevice::Uart6 as usize] };
    device.set_baud(rate);
}


pub fn serial_write(device: &SerioDevice, bytes: &[u8]) {
    let device_idx = *device as usize;

    // Check for uart5 because idk how to support that one
    match device {
        SerioDevice::Uart5 => {
            panic!("Uart5 is not supported");
        },
        _ => {
            
        },
    };
    
    let uart = unsafe { &mut UART_DEVICES[device_idx] };
    if !uart.initialized {
        uart.initialize();
    }

    uart.write(bytes);
}

pub fn uart_irq_handler() {
    let mut device_id = 0;
    let uart_count = unsafe { UART_DEVICES.len() };

    // Map over each device and tell it an irq event happened.
    while device_id < uart_count {
        let device = unsafe { &mut UART_DEVICES[device_id] };
        device.handle_irq();
        device_id += 1;
    }
}