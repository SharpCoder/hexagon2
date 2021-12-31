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

// The device serial communication is hardcoded tos
pub const SERIO_DEV: Device = Device::Uart6;
pub const DMA_TX_CHANNEL: u32 = 0;
pub const DMA_RX_CHANNEL: u32 = 1;

pub const CTSB_PIN: usize = 2;
pub const TX_PIN: usize = 1;
pub const RX_PIN: usize = 0;

static mut transmitting: bool = false;
static mut t: bool = false;

pub static mut offset: u32 = 0x0;

static mut buff_idx: u32 = 0;
static mut buff: [u8; 6] = [
    b'h',
    b'e',
    b'l',
    b'l',
    b'o',
    b'\n',
];

pub fn serio_init() {
    // Do some muxing
    // TX
    pin_mux_config(TX_PIN, Alt::Alt2); // LPUART6 alternative
    pin_pad_config(TX_PIN, PadConfig {
        hysterisis: true,
        resistance: PullUpDown::PullDown100k,
        pull_keep: PullKeep::Keeper,
        pull_keep_en: false,
        open_drain: false,
        speed: PinSpeed::Max200MHz,
        drive_strength: DriveStrength::MaxDiv3,
        fast_slew_rate: true,
    });

    // RX
    // pin_mux_config(RX_PIN, Alt::Alt2); // LPUART6 Rx Alternative
    // pin_pad_config(RX_PIN, PadConfig {
    //     hysterisis: false,
    //     resistance: PullUpDown::PullUp100k,
    //     pull_keep: PullKeep::Keeper,
    //     pull_keep_en: false,
    //     open_drain: false,
    //     speed: PinSpeed::Max200MHz,
    //     drive_strength: DriveStrength::MaxDiv3,
    //     fast_slew_rate: true,
    // });


    uart_disable(&SERIO_DEV);
    uart_sw_reset(&SERIO_DEV, true);
    uart_sw_reset(&SERIO_DEV, false);

    uart_configure(&SERIO_DEV, UartConfig {
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

    uart_configure_fifo(&SERIO_DEV, FifoConfig {
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

    uart_set_pin_config(&SERIO_DEV, InputTrigger::Disabled);
    uart_disable_fifo(&SERIO_DEV);

    attach_irq(Irq::UART6, uart_irq_handler);
    irq_enable(Irq::UART6);

    // TX
    // attach_irq(Irq::EDMA0, serio_irq_handler);
    // irq_enable(Irq::EDMA0);

    // RX
    // attach_irq(Irq::EDMA1, serio_irq_handler);
    // irq_enable(Irq::EDMA1);

    // fill_irq(serio_irq_handler);

    // uart_enable_dma(&SERIO_DEV);
    uart_watermark(&SERIO_DEV);
    uart_enable(&SERIO_DEV);

    // Enable the transmitter
    // pin_mode(TX_PIN, Mode::Output);

    // Configure CTS
    // serio_attach_cts();
    serio_transmit_enable();

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
    let r: u32 = 0;
    unsafe {
        offset = crate::ptr_to_addr_word(&buff_idx);
    }

}

fn serio_attach_cts() {
    let pin = CTSB_PIN;
    let xbar_in_index = 6;
    let xbar_out_index = 124;

    xbar_connect(xbar_in_index, xbar_out_index);

    // Pin configure, trigger select
    uart_set_pin_config(&SERIO_DEV, InputTrigger::CtsB);

    // Configure something else
    pin_mode(pin, Mode::Output);
    assign(0x401F_861C, 0x0);
    // pin_mux_config(pin, Alt::Alt1);
    // pin_pad_config(pin, Bitwise::Eq, (0x7 << 3) | (0x1 << 12) | (0x1 << 13) | (0x0 << 14) | (0x1 << 16));
}

fn serio_transmit_enable() {
    pin_mode(TX_PIN, Mode::Output);
    pin_out(TX_PIN, Power::Low);
}

pub fn serio_baud(rate: f32) {
    uart_baud_rate(&SERIO_DEV, rate);
}

pub fn serio_write_byte(byte: u8) {

    unsafe {
        if transmitting == false {
            pin_out(TX_PIN, Power::High);
            // uart_write_fifo(&SERIO_DEV, b'H');
            uart_set_tie(&SERIO_DEV, true);
            transmitting = true;
        }
    }
    
    // let cidx = unsafe { char_send_idx };
    // if cidx < 6 {
    //     uart_write_fifo(&SERIO_DEV, byte);
    //     unsafe { char_send_idx += 1; }
    //     uart_set_tie(&SERIO_DEV, true);
    // } else {
    //     uart_flush(&SERIO_DEV);
    // }

    // disable_interrupts();
    // let buffer: [u8; 1] = [byte];
    // let addr = unsafe {
    //     crate::ptr_to_addr_byte(buffer.as_ptr())
    // };

    // dma_source_buffer(DMA_TX_CHANNEL, addr, 1);
    // dma_enable_request(DMA_TX_CHANNEL);
}

pub fn uart_irq_handler() {

    // pin_out(13, Power::Low);
    // debug::blink(4, debug::Speed::Fast);
    // if uart_get_irq_statuses(&SERIO_DEV) & (0x1 << 22) > 0 {
        // uart_disable(&SERIO_DEV);
        // uart_enable(&SERIO_DEV);
        // uart_sbk(&SERIO_DEV);
        // pin_out(TX_PIN, Power::Low);
    // }    
    
    // disable_interrupts();
    // uart_set_tie(&SERIO_DEV, false);

    // Tx empty, I think
    if uart_get_irq_statuses(&SERIO_DEV) & (0x1 << 23) > 0 {
        // debug::blink(1, debug::Speed::Normal);
        

        let mut byte: u8 = b'a';
        

        // while adj > 255 {
        //     adj -= 255;
        // }
        uart_disable(&SERIO_DEV);
        uart_enable(&SERIO_DEV);
        uart_sbk(&SERIO_DEV);
        uart_write_fifo(&SERIO_DEV, byte);
        // uart_disable(&SERIO_DEV);
        // uart_enable(&SERIO_DEV);
        // uart_sbk(&SERIO_DEV);

        // unsafe {
            
        //     buff_idx = (idx + 1) % 5;
        // }
        // }
    }
    
    // uart_set_tie(&SERIO_DEV, true);
    uart_clear_irq(&SERIO_DEV);
    // enable_interrupts();
}

#[no_mangle]
pub fn serio_irq_handler() {
    dma_clear_irq(DMA_TX_CHANNEL);
    dma_clear_irq(DMA_RX_CHANNEL);
    unsafe {
        asm!("nop");
    }
}