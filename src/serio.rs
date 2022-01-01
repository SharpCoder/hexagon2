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

const UART_BUFFER_SIZE: usize = 1024; // bytes
static mut Uart1: Uart = Uart::new(Device::Uart1, /* TX Pin */ 24, /* RX Pin */ 25, /* IRQ */ Irq::UART1);
static mut Uart2: Uart = Uart::new(Device::Uart2, 14, 15, Irq::UART2);
static mut Uart3: Uart = Uart::new(Device::Uart3, 17, 16, Irq::UART3);
static mut Uart4: Uart = Uart::new(Device::Uart4, 8, 7, Irq::UART4);
static mut Uart5: Uart = Uart::new(Device::Uart5, 1, 0, Irq::UART5); // NOTE: THIS DEVICE DOESN'T HAVE VALID PINS
static mut Uart6: Uart = Uart::new(Device::Uart6, 1, 0, Irq::UART6);
static mut Uart7: Uart = Uart::new(Device::Uart7, 29, 28, Irq::UART7);
static mut Uart8: Uart = Uart::new(Device::Uart8, 20, 21, Irq::UART8);

#[derive(Clone, Copy)]
pub enum SerioDevice {
    Uart1 = 0x0,
    Uart2 = 0x1,
    Uart3 = 0x2,
    Uart4 = 0x3,
    Uart5 = 0x4,
    Uart6 = 0x5,
    Uart7 = 0x6,
    Uart8 = 0x7,
}


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
        
        uart_disable(self.device);
        uart_sw_reset(self.device, true);
        uart_sw_reset(self.device, false);
        uart_configure(self.device, UartConfig {
            r9t8: false,
            invert_transmission_polarity: false,
            overrun_irq_en: false,
            noise_error_irq_en: false,
            framing_error_irq_en: false,
            parity_error_irq_en: false,
            tx_irq_en: false,
            rx_irq_en: false,
            tx_complete_irq_en: false, // true
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
    
        uart_configure_fifo(self.device, FifoConfig {
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
    
        pin_mode(self.tx_pin, Mode::Output);
        uart_set_pin_config(self.device, InputTrigger::Disabled);
        uart_disable_fifo(self.device);
        
        uart_watermark(self.device);
        uart_enable(self.device);
        
        pin_out(self.tx_pin, Power::Low);
        
        attach_irq(self.irq, serio_handle_irq);
        fill_irq(serio_handle_irq);
        irq_enable(self.irq);

        self.initialized = true;        
    }

    pub fn set_baud(&self, rate: f32) {
        uart_baud_rate(self.device, rate);
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
        // Make sure we're not buffer overflowed
        if (self.buffer_head + 1) >= UART_BUFFER_SIZE {
            return;
        }

        self.buffer[self.buffer_head] = byte;
        self.buffer_head += 1;

        if self.buffer_head == 1 {
            pin_out(self.tx_pin, Power::High);
            // uart_write_fifo(self.device, byte);
            uart_set_reg(self.device, &CTRL_TCIE);
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

        // If tx is empty
        if uart_get_irq_statuses(self.device) & (0x1 << 23) > 0 {
            match self.dequeue() {
                None => {
                    // Disengage, I guess?
                    uart_clear_reg(self.device, &CTRL_TCIE);
                    
                },
                Some(byte) => {
                    // Clear TSC
                    uart_disable(self.device);
                    uart_enable(self.device);
                    uart_sbk(self.device);

                    // Get the next byte to write and beam it
                    uart_write_fifo(self.device, byte);
                }
            }
        }

        self.irq_processing = false;
        uart_clear_irq(self.device);
    }
}

fn get_uart_interface (device: SerioDevice) -> &'static mut Uart {
    unsafe {
        return match device {
            SerioDevice::Uart1 => &mut Uart1,
            SerioDevice::Uart2 => &mut Uart2,
            SerioDevice::Uart3 => &mut Uart3,
            SerioDevice::Uart4 => &mut Uart4,
            SerioDevice::Uart5 => &mut Uart5,
            SerioDevice::Uart6 => &mut Uart6,
            SerioDevice::Uart7 => &mut Uart7,
            SerioDevice::Uart8 => &mut Uart8,
        };
    }
}

pub fn serio_init() {
    fill_irq(serio_handle_irq);
    let uart = get_uart_interface(SerioDevice::Uart6);
    uart.initialize();
}

pub fn serio_write(bytes: &[u8]) {
    serial_write(SerioDevice::Uart6, bytes);
}

pub fn serial_write(device: SerioDevice, bytes: &[u8]) {
    let uart = get_uart_interface(device);
    unsafe { uart.write(bytes); }
}

pub fn serio_baud(rate: f32) {
    uart_baud_rate(Device::Uart6, rate);
}

#[no_mangle]
pub fn serio_handle_irq() {
    disable_interrupts();
    get_uart_interface(SerioDevice::Uart1).handle_irq();
    get_uart_interface(SerioDevice::Uart2).handle_irq();
    get_uart_interface(SerioDevice::Uart3).handle_irq();
    get_uart_interface(SerioDevice::Uart4).handle_irq();
    get_uart_interface(SerioDevice::Uart5).handle_irq();
    get_uart_interface(SerioDevice::Uart6).handle_irq();
    enable_interrupts();
}