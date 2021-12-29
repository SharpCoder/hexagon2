/** 
 * This module represents the serial communication protocol
 * based on UART physical hardware. For simplicity, it is tightly
 * coupled to a specific uart device.
*/
use core::arch::asm;
use crate::phys::addrs;
use crate::phys::uart::*;
use crate::phys::irq::*;
use crate::phys::dma::*;
use crate::phys::*;

// The device serial communication is hardcoded tos
pub const SERIO_DEV: Device = Device::Uart1;
pub const DMA_CHANNEL: u32 = 0;

pub fn serio_init() {
    // Do some muxing
    // TX
    assign(addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_AD_B0_12, 0x2);
    assign(addrs::IOMUXC_SW_PAD_CTL_PAD_GPIO_AD_B0_12, 0x78);

    // RX
    assign(addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_AD_B0_14, 0x2);
    assign(addrs::SW_MUX_CTL_PAD_GPIO_B0_14, 0x2);

    uart_sw_reset(&SERIO_DEV, true);
    uart_disable(&SERIO_DEV);

    uart_configure(&SERIO_DEV, UartConfig {
        invert_transmission_polarity: false,
        overrun_irq_en: false,
        noise_error_irq_en: false,
        framing_error_irq_en: false,
        parity_error_irq_en: false,
        tx_irq_en: false,
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
        tx_fifo_overflow_irq_en: false,
        rx_fifo_underflow_irq_en: false,
        tx_fifo_en: false,
        tx_fifo_depth: BufferDepth::Data1Word,
        rx_fifo_en: false,
        rx_fifo_depth: BufferDepth::Data1Word,
    });

    uart_set_pin_config(&SERIO_DEV, 0x0);
    uart_disable_fifo(&SERIO_DEV);

    attach_irq(Irq::UART1, serio_irq_handler);
    irq_enable(Irq::UART1);

    attach_irq(Irq::EDMA0, serio_irq_handler);
    irq_enable(Irq::EDMA0);

    // fill_irq(serio_irq_handler);

    uart_enable_dma(&SERIO_DEV);
    uart_watermark(&SERIO_DEV);
    // uart_enable(&SERIO_DEV);


    // Configure DMA
    dma_configure_source(DMA_CHANNEL, DMASource::Uart1Tx);
    dma_destination(DMA_CHANNEL, 0x4018_4000 + 0x1C);
    dma_enable_irq(DMA_CHANNEL);
    dma_interrupt_at_completion(DMA_CHANNEL);
    dma_enable(DMA_CHANNEL);
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
    let buffer: [u8; 1] = [byte];
    let addr = unsafe {
        crate::ptr_to_addr_byte(buffer.as_ptr())
    };

    dma_source_buffer(DMA_CHANNEL, addr, 1);
}

#[no_mangle]
pub fn serio_irq_handler() {

    // Read the interrupts
    if dma_is_irq(DMA_CHANNEL) {
        crate::debug::blink(1, crate::debug::Speed::Slow);
    }

    // uart_clear_irq(&SERIO_DEV);
    dma_clear_irq(DMA_CHANNEL);
    // dma_clear_done_status(DMA_CHANNEL);
    unsafe {
        asm!("nop");
    }
}