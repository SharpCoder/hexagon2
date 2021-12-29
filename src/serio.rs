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
use crate::phys::pins::*;
use crate::phys::*;

// The device serial communication is hardcoded tos
pub const SERIO_DEV: Device = Device::Uart6;
pub const DMA_TX_CHANNEL: u32 = 0;
pub const DMA_RX_CHANNEL: u32 = 1;
static mut transmitting: bool = false;

fn is_transmitting() -> bool {
    unsafe {
        return transmitting;
    }
}

pub fn set_transmitting(val: bool) {
    unsafe {
        transmitting = val;
    }
}

pub fn serio_init() {
    // Do some muxing
    // TX
    pin_mux_config(1, Alt::Alt2);
    pin_pad_config(1, Bitwise::Eq, PCR_SRE | (0x7 << 3) | (0x3 << 6));
    
    // RX
    assign(addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_AD_B0_03, 0x2);
    assign(addrs::IOMUXC_SW_PAD_CTL_PAD_GPIO_AD_B0_03, 0x2);
    assign(0x401F8400 + 0x150, 0x1);

    // uart_sw_reset(&SERIO_DEV, true);
    uart_disable(&SERIO_DEV);

    uart_configure(&SERIO_DEV, UartConfig {
        r9t8: false,
        invert_transmission_polarity: false,
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
        tx_flush: true,
        rx_flush: false,
        tx_fifo_overflow_irq_en: false,
        rx_fifo_underflow_irq_en: false,
        tx_fifo_en: true,
        tx_fifo_depth: BufferDepth::Data4Words,
        rx_fifo_en: false,
        rx_fifo_depth: BufferDepth::Data4Words,
    });

    uart_set_pin_config(&SERIO_DEV, 0x0);
    uart_disable_fifo(&SERIO_DEV);

    attach_irq(Irq::UART6, uart_irq_handler);
    irq_enable(Irq::UART6);

    // TX
    attach_irq(Irq::EDMA0, serio_irq_handler);
    irq_enable(Irq::EDMA0);

    // RX
    attach_irq(Irq::EDMA1, serio_irq_handler);
    irq_enable(Irq::EDMA1);

    // fill_irq(serio_irq_handler);

    // uart_enable_dma(&SERIO_DEV);
    uart_watermark(&SERIO_DEV);
    uart_enable(&SERIO_DEV);


    // // Configure rx DMA
    // dma_configure_source(DMA_RX_CHANNEL, DMASource::Uart6Rx);
    // dma_source_addr(DMA_RX_CHANNEL, 0x4018_4000 + 0x1C);
    // dma_enable_irq(DMA_RX_CHANNEL);
    // dma_enable(DMA_RX_CHANNEL);


    // // Configure tx DMA
    // dma_configure_source(DMA_TX_CHANNEL, DMASource::Uart6Tx);
    // dma_dest_addr(DMA_TX_CHANNEL, 0x4018_4000 + 0x1C);
    // dma_enable_irq(DMA_TX_CHANNEL);
    // dma_interrupt_at_completion(DMA_TX_CHANNEL);
    // dma_disable_on_completion(DMA_TX_CHANNEL);
    // dma_enable(DMA_TX_CHANNEL);
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
    if !is_transmitting() {
        uart_write_fifo(&SERIO_DEV, byte);
        uart_flush(&SERIO_DEV);
    }

    // disable_interrupts();
    // let buffer: [u8; 1] = [byte];
    // let addr = unsafe {
    //     crate::ptr_to_addr_byte(buffer.as_ptr())
    // };

    // dma_source_buffer(DMA_TX_CHANNEL, addr, 1);
    // dma_enable_request(DMA_TX_CHANNEL);
    // enable_interrupts();
}

pub fn uart_irq_handler() {
    if uart_get_irq_statuses(&SERIO_DEV) & (0x1 << 22) > 0 {
        crate::debug::blink(3, crate::debug::Speed::Fast);
        // uart_disable(&SERIO_DEV);
        // uart_enable(&SERIO_DEV);
        // uart_sbk(&SERIO_DEV);
        // set_transmitting(false);
    }
    
    uart_clear_irq(&SERIO_DEV);
}

#[no_mangle]
pub fn serio_irq_handler() {
    dma_clear_irq(DMA_TX_CHANNEL);
    dma_clear_irq(DMA_RX_CHANNEL);
    unsafe {
        asm!("nop");
    }
}