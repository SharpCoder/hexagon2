use crate::debug::*;

/** 
 * This module represents the serial communication protocol
 * based on UART physical hardware. For simplicity, it is tightly
 * coupled to a specific uart device.
*/
use core::arch::asm;
use crate::phys::uart::*;
use crate::phys::irq::*;
use crate::phys::pins::*;
use crate::phys::addrs;
use crate::datastructures::*;

struct HardwareConfig {
    device: Device,
    tx_pin: usize,
    rx_pin: usize,
    irq: Irq,
    sel_inp_reg: Option<u32>,
    sel_inp_val: Option<u32>,
}

const UART_WATERMARK_SIZE: u32 = 0x2;
const UART_BUFFER_SIZE: usize = 128; // bytes
static mut UART1: Uart = Uart::new(HardwareConfig {
    device: Device::Uart1,
    tx_pin: 24,
    rx_pin: 25,
    irq: Irq::Uart1,
    sel_inp_reg: None,
    sel_inp_val: None,
 });
 
 static mut UART2: Uart = Uart::new(HardwareConfig {
    device: Device::Uart2, 
    tx_pin: 14, 
    rx_pin: 15, 
    irq: Irq::Uart2,
    sel_inp_reg: Some(addrs::IOMUXC_LPUART2_RX_SELECT_INPUT),
    sel_inp_val: Some(0x1),
 });

static mut UART3: Uart = Uart::new(HardwareConfig {
    device: Device::Uart3, 
    tx_pin: 17, 
    rx_pin: 16, 
    irq: Irq::Uart3,
    sel_inp_reg: Some(addrs::IOMUXC_LPUART3_RX_SELECT_INPUT),
    sel_inp_val: Some(0x0),
});

static mut UART4: Uart = Uart::new(HardwareConfig {
    device: Device::Uart4, 
    tx_pin: 8, 
    rx_pin: 7, 
    irq: Irq::Uart4,
    sel_inp_reg: Some(addrs::IOMUXC_LPUART4_RX_SELECT_INPUT),
    sel_inp_val: Some(0x2),
});

static mut UART5: Uart = Uart::new(HardwareConfig {
    device: Device::Uart5, 
    tx_pin: 1, 
    rx_pin: 0, 
    irq: Irq::Uart5,
    sel_inp_reg: Some(addrs::IOMUXC_LPUART5_RX_SELECT_INPUT),
    sel_inp_val: Some(0x0),
}); // NOTE: THIS DEVICE DOESN'T HAVE VALID PINS

static mut UART6: Uart = Uart::new(HardwareConfig {
    device: Device::Uart6, 
    tx_pin: 1, 
    rx_pin: 0, 
    irq: Irq::Uart6,
    sel_inp_reg: Some(addrs::IOMUXC_LPUART6_RX_SELECT_INPUT),
    sel_inp_val: Some(0x1),
});

static mut UART7: Uart = Uart::new(HardwareConfig {
    device: Device::Uart7, 
    tx_pin: 29, 
    rx_pin: 28, 
    irq: Irq::Uart7,
    sel_inp_reg: Some(addrs::IOMUXC_LPUART7_RX_SELECT_INPUT),
    sel_inp_val: Some(0x1),
});

static mut UART8: Uart = Uart::new(HardwareConfig {
    device: Device::Uart8, 
    tx_pin: 20, 
    rx_pin: 21, 
    irq: Irq::Uart8,
    sel_inp_reg: Some(addrs::IOMUXC_LPUART8_RX_SELECT_INPUT),
    sel_inp_val: Some(0x0),
});

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
    Default = 0x8,
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
    irq: Irq,
    tx_buffer: Buffer::<UART_BUFFER_SIZE, u8>,
    rx_buffer: Buffer::<UART_BUFFER_SIZE, u8>,
    sel_inp_reg: Option<u32>,
    sel_inp_val: Option<u32>,
    buffer_head: usize,
    tx_count: u32,
}

impl Uart {
    pub const fn new(config: HardwareConfig) -> Uart {
        return Uart {
            device: config.device,
            tx_buffer: Buffer::<UART_BUFFER_SIZE, u8> {
                data: [0; UART_BUFFER_SIZE],
                tail: 0,
            },
            rx_buffer: Buffer::<UART_BUFFER_SIZE, u8> {
                data: [0; UART_BUFFER_SIZE],
                tail: 0,
            },
            buffer_head: 0,
            initialized: false,
            tx_pin: config.tx_pin,
            rx_pin: config.rx_pin,
            sel_inp_reg: config.sel_inp_reg,
            sel_inp_val: config.sel_inp_val,
            irq: config.irq,
            tx_count: UART_WATERMARK_SIZE,
        }
    }

    fn initialize(&mut self) {
        if self.initialized {
            return;
        }
        
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

        pin_mux_config(self.rx_pin, Alt::Alt2);
        pin_pad_config(self.rx_pin, PadConfig {
            hysterisis: true,
            resistance: PullUpDown::PullUp22k,
            pull_keep: PullKeep::Pull,
            pull_keep_en: true,
            open_drain: false,
            speed: PinSpeed::Low50MHz,
            drive_strength: DriveStrength::Max,
            fast_slew_rate: false,
        });

        // Configure the base settings
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
            tx_irq_en: false, // This gets set later
            rx_irq_en: false,
            tx_complete_irq_en: false,
            idle_line_irq_en: false,
            tx_en: false,
            rx_en: false,
            match1_irq_en: false,
            match2_irq_en: false,
            idle_config: IdleConfiguration::Idle32Char,
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
            tx_fifo_depth: BufferDepth::Data64Words,
            rx_fifo_en: true,
            rx_fifo_depth: BufferDepth::Data64Words,
        });


        uart_set_pin_config(self.device, InputTrigger::Disabled);
        uart_disable_fifo(self.device);
        
        uart_watermark(self.device, UART_WATERMARK_SIZE);
        uart_enable(self.device);

        pin_mode(self.tx_pin, Mode::Output);
        pin_mode(self.rx_pin, Mode::Input);

        // If this uart requires additional input muxing, do it.
        if self.sel_inp_reg.is_some() {
            crate::phys::assign(self.sel_inp_reg.unwrap(), self.sel_inp_val.unwrap());
        }

        pin_out(self.tx_pin, Power::Low);
        
        irq_attach(self.irq, serio_handle_irq);
        irq_enable(self.irq);

        uart_baud_rate(self.device, 115200);


        self.initialized = true;        
    }

    pub fn available(&self) -> usize {   
        if self.rx_buffer.size() > 0 {
            return self.rx_buffer.size();
        } else if uart_has_data(self.device) {
            return 1;
        } else {
            return 0;
        }
    }

    pub fn write(&mut self, bytes: &[u8]) {
        disable_interrupts();
        for byte_idx in 0 .. bytes.len() {
            self.tx_buffer.enqueue(bytes[byte_idx]);
        }

        pin_out(self.tx_pin, Power::High);
        uart_set_reg(self.device, &CTRL_TIE);
        enable_interrupts();
    }

    pub fn write_vec(&mut self, bytes: Vector<u8>) {
        disable_interrupts();
        for byte_idx in 0 .. bytes.size() {
            match bytes.get(byte_idx) {
                None => { break; },
                Some(byte) => {
                    self.tx_buffer.enqueue(byte);
                }
            }
        }

        pin_out(self.tx_pin, Power::High);
        uart_set_reg(self.device, &CTRL_TIE);
        enable_interrupts();
    }

    pub fn get_rx_buffer(&self) -> &[u8] {
        return self.rx_buffer.as_array();
    }

    pub fn clear_rx_buffer(&mut self) {
        self.rx_buffer.clear();
    }

    pub fn read(&mut self) -> Option<u8> {
        if self.rx_buffer.size() > 0 {
            return self.rx_buffer.dequeue();
        } else if uart_has_data(self.device) {
            return Some(uart_read_fifo(self.device));
        } else {
            return None;
        }
    }

    fn handle_receive_irq(&mut self) {
        let irq_statuses = uart_get_irq_statuses(self.device);
        
        
        // If data register is full
        if uart_has_data(self.device) || irq_statuses & (0x1 << 21) > 0 || irq_statuses & (0x1 << 20) > 0 {
            blink_accumulate();
            debug_hex(irq_statuses, b"IRQ");

            // Read until it is empty
            while uart_has_data(self.device) {
                let msg: u8 = uart_read_fifo(self.device);
                self.rx_buffer.enqueue(msg);
            }
            
            uart_clear_irq(self.device, UartClearIrqConfig {
                rx_data_full: true,
                rx_idle: true,
                rx_line_break: true,
                rx_overrun: true,
                rx_pin_active: true,
                rx_set_data_inverted: false,
                tx_complete: false,
                tx_empty: false,
            });
        }
    }
    
    fn transmit(&mut self) {
        // uart_disable(self.device);
        // uart_enable(self.device);
        // uart_sbk(self.device);
        // uart_queue_preamble(self.device);

        match self.tx_buffer.dequeue() {
            None => {
            },
            Some(byte) => {
                // Get the next byte to write and beam it
                uart_write_fifo(self.device, byte);
            }
        }
        
        self.tx_count += 1;

        // Activate TCIE (Transmit Complete Interrupt Enable)
        uart_set_reg(self.device, &CTRL_TCIE);

        // crate::wait_ns(crate::MS_TO_NANO);
    }

    fn handle_send_irq(&mut self) {
        // blink_accumulate();

        // Transmission complete
        let irq_statuses = uart_get_irq_statuses(self.device);
        let tx_complete = irq_statuses & (0x1 << 22) > 0;
        let tx_empty = irq_statuses & (0x1 << 23) > 0;
        let pending_data = self.tx_buffer.size() > 0;

        if tx_complete && self.tx_count > UART_WATERMARK_SIZE {
            self.tx_count -= 1;
        }

        // Check if there is space in the buffer
        if pending_data {
            for i in self.tx_count .. uart_get_tx_size(self.device) {
                self.transmit();
            }
        } else if !pending_data {
            // blink_hardware(1);
            // Disengage, I guess?
            uart_clear_reg(self.device, &CTRL_TIE);
            uart_clear_reg(self.device, &CTRL_TCIE);

            uart_clear_irq(self.device, UartClearIrqConfig {
                rx_data_full: false,
                rx_idle: false,
                rx_line_break: false,
                rx_overrun: false,
                rx_pin_active: false,
                rx_set_data_inverted: false,
                tx_complete: true,
                tx_empty: true,
            });
        }

        // If tx is empty
        // if uart_get_irq_statuses(self.device) & (0x1 << 23) > 0 {
        //     self.transmit();
            
        //     // Isn't timing the whole point of UART?
        //     // I must be doing something wrong...
        //     // crate::wait_ns(crate::MS_TO_NANO * 1);
            // uart_clear_irq(self.device, UartClearIrqConfig {
            //     rx_data_full: false,
            //     rx_idle: false,
            //     rx_line_break: false,
            //     rx_overrun: false,
            //     rx_pin_active: false,
            //     rx_set_data_inverted: false,
            //     tx_complete: false,
            //     tx_empty: false,
            // });
        // }
    }

    pub fn handle_irq(&mut self) {
        // Don't process a uart device that hasn't
        // been used
        if !self.initialized {
            return;
        }

        // This prevents circular calls
        disable_interrupts();
        // self.handle_receive_irq();
        self.handle_send_irq();
        enable_interrupts();
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

            // Specify defaut here
            SerioDevice::Default => &mut UART6,
        };
    }
}

pub fn serial_init(device: SerioDevice) {
    let uart = get_uart_interface(device);
    uart.initialize();
}

pub fn serial_read(device: SerioDevice) -> Option<u8> {
    let uart = get_uart_interface(device);
    return uart.read();
}

pub fn serial_available(device: SerioDevice) -> usize {
    let uart = get_uart_interface(device);
    return uart.available();
}

pub fn serial_write(device: SerioDevice, bytes: &[u8]) {
    let uart = get_uart_interface(device);
    uart.initialize();
    uart.write(bytes);
}

pub fn serial_write_vec(device: SerioDevice, bytes: Vector<u8>) {
    let uart = get_uart_interface(device);
    uart.initialize();
    uart.write_vec(bytes);
}

pub fn serial_buffer<'a>(device: SerioDevice) -> &'a [u8] {
    let uart = get_uart_interface(device);
    return uart.get_rx_buffer();
}

pub fn serial_clear_rx(device: SerioDevice) {
    let uart = get_uart_interface(device);
    uart.clear_rx_buffer();
}

pub fn serio_baud(rate: u32) {
    uart_baud_rate(Device::Uart6, rate);
}

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