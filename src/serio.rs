/** 
 * This module represents the serial communication protocol
 * based on UART physical hardware. For simplicity, it is tightly
 * coupled to a specific uart device.
*/
use crate::phys::addrs;
use crate::phys::uart::*;
use crate::phys::irq::*;
use crate::phys::*;

// The device serial communication is hardcoded tos
pub const SERIO_DEV: Device = Device::Uart1;

pub fn serio_init() {

    // Do some muxing
    assign(addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_AD_B0_12, 0x2);
    assign(addrs::IOMUXC_SW_PAD_CTL_PAD_GPIO_AD_B0_12, 0x78);

    uart_sw_reset(&SERIO_DEV, true);
    uart_disable(&SERIO_DEV);
    uart_configure(&SERIO_DEV, UartConfig {
        invert_transmission_polarity: true,
        overrun_irq_en: false,
        noise_error_irq_en: false,
        framing_error_irq_en: false,
        parity_error_irq_en: false,
        tx_irq_en: true,
        rx_irq_en: false,
        tx_complete_irq_en: false,
        idle_line_irq_en: false,
        tx_en: false,
        rx_en: false,
        match1_irq_en: false,
        match2_irq_en: false,
        idle_config: IdleConfiguration::Idle1Char,
        doze_en: false,
        bit_mode: BitMode::EightBits,
        parity_en: false,
        parity_type: ParityType::Odd,
    });

    uart_configure_fifo(&SERIO_DEV, FifoConfig {
        tx_fifo_underflow_flag: false,
        rx_fifo_underflow_flag: false,
        tx_flush: false,
        rx_flush: false,
        tx_fifo_overflow_irq_en: true,
        rx_fifo_underflow_irq_en: false,
        tx_fifo_en: true,
        tx_fifo_depth: BufferDepth::Data1Word,
        rx_fifo_en: false,
        rx_fifo_depth: BufferDepth::Data1Word,
    });

    attach_irq(Irq::UART1, serio_irq_handler);
    irq_enable(Irq::UART1);
    attach_irq(Irq::UART2, serio_irq_handler);
    irq_enable(Irq::UART2);
    attach_irq(Irq::UART3, serio_irq_handler);
    irq_enable(Irq::UART3);
    attach_irq(Irq::UART4, serio_irq_handler);
    irq_enable(Irq::UART4);
    attach_irq(Irq::UART5, serio_irq_handler);
    irq_enable(Irq::UART5);
    attach_irq(Irq::UART6, serio_irq_handler);
    irq_enable(Irq::UART6);
    attach_irq(Irq::UART7, serio_irq_handler);
    irq_enable(Irq::UART7);
    attach_irq(Irq::UART8, serio_irq_handler);
    irq_enable(Irq::UART8);

    attach_irq(Irq::EDMA1, serio_irq_handler);
    irq_enable(Irq::EDMA1);
    attach_irq(Irq::EDMA2, serio_irq_handler);
    irq_enable(Irq::EDMA2);
    attach_irq(Irq::EDMA3, serio_irq_handler);
    irq_enable(Irq::EDMA3);
    attach_irq(Irq::EDMA4, serio_irq_handler);
    irq_enable(Irq::EDMA4);
    attach_irq(Irq::EDMA5, serio_irq_handler);
    irq_enable(Irq::EDMA5);

    fill_irq(serio_irq_handler);

    uart_watermark(&SERIO_DEV);
    uart_enable(&SERIO_DEV);
}

pub fn serio_baud(rate: Baud) {
    uart_baud_rate(&SERIO_DEV, rate);
}

pub fn serio_write(string: &[u8]) {
    let mut idx = 0;
    while idx < string.len() {
        serio_write_byte(string[idx]);
        idx += 1;
    }
}

pub fn serio_write_byte(byte: u8) {
    uart_write_fifo(&SERIO_DEV, byte);
    uart_flush(&SERIO_DEV);
}

#[no_mangle]
pub fn serio_irq_handler() {
    crate::debug::blink(5, crate::debug::Speed::Fast);
}