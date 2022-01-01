/** 
 * This module represents the serial communication protocol
 * based on UART physical hardware. For simplicity, it is tightly
 * coupled to a specific uart device.
*/
use crate::phys::uart::*;
use crate::phys::irq::*;
use crate::phys::pins::*;

const UART_BUFFER_SIZE: usize = 1024; // bytes
static mut UART1: Uart = Uart::new(Device::Uart1, /* TX Pin */ 24, /* RX Pin */ 25, /* IRQ */ Irq::Uart1);
static mut UART2: Uart = Uart::new(Device::Uart2, 14, 15, Irq::Uart2);
static mut UART3: Uart = Uart::new(Device::Uart3, 17, 16, Irq::Uart3);
static mut UART4: Uart = Uart::new(Device::Uart4, 8, 7, Irq::Uart4);
static mut UART5: Uart = Uart::new(Device::Uart5, 1, 0, Irq::Uart5); // NOTE: THIS DEVICE DOESN'T HAVE VALID PINS
static mut UART6: Uart = Uart::new(Device::Uart6, 1, 0, Irq::Uart6);
static mut UART7: Uart = Uart::new(Device::Uart7, 29, 28, Irq::Uart7);
static mut UART8: Uart = Uart::new(Device::Uart8, 20, 21, Irq::Uart8);

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
        
        irq_attach(self.irq, serio_handle_irq);
        irq_enable(self.irq);

        uart_baud_rate(self.device, 9600.0);

        self.initialized = true;        
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
            SerioDevice::Uart1 => &mut UART1,
            SerioDevice::Uart2 => &mut UART2,
            SerioDevice::Uart3 => &mut UART3,
            SerioDevice::Uart4 => &mut UART4,
            SerioDevice::Uart5 => &mut UART5,
            SerioDevice::Uart6 => &mut UART6,
            SerioDevice::Uart7 => &mut UART7,
            SerioDevice::Uart8 => &mut UART8,
        };
    }
}

pub fn serio_init() {
    // fill_irq(serio_handle_irq);
    // put_irq(0, serio_handle_irq);
    // put_irq(1, serio_handle_irq);
    // put_irq(2, serio_handle_irq);
    // put_irq(3, serio_handle_irq);
    // put_irq(4, serio_handle_irq);
    // put_irq(5, serio_handle_irq);
    // put_irq(6, serio_handle_irq);
    // put_irq(7, serio_handle_irq);
    // put_irq(20, serio_handle_irq);
    // put_irq(21, serio_handle_irq);
    // put_irq(22, serio_handle_irq);
    // put_irq(23, serio_handle_irq);
    // put_irq(24, serio_handle_irq);
    // put_irq(25, serio_handle_irq);
    // fill_irq(serio_handle_irq);
    let uart = get_uart_interface(SerioDevice::Uart6);
    uart.initialize();
}

pub fn serio_write(bytes: &[u8]) {
    serial_write(SerioDevice::Uart6, bytes);
    serial_write(SerioDevice::Uart4, bytes);
}

pub fn serial_write(device: SerioDevice, bytes: &[u8]) {
    let uart = get_uart_interface(device);
    uart.write(bytes);
}

pub fn serio_baud(rate: f32) {
    uart_baud_rate(Device::Uart6, rate);
}

#[no_mangle]
pub fn serio_handle_irq() {
    // debug::blink(2, crate::debug::Speed::Fast);
    disable_interrupts();
    get_uart_interface(SerioDevice::Uart1).handle_irq();
    get_uart_interface(SerioDevice::Uart2).handle_irq();
    get_uart_interface(SerioDevice::Uart3).handle_irq();
    get_uart_interface(SerioDevice::Uart4).handle_irq();
    get_uart_interface(SerioDevice::Uart5).handle_irq();
    get_uart_interface(SerioDevice::Uart6).handle_irq();
    enable_interrupts();
}