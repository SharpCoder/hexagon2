use crate::phys::*;
use crate::phys::addrs;

pub enum DMASource {
    Uart1Tx = 2,
    Uart1Rx = 3,
    Uart3Tx = 4,
    Uart3Rx = 5,
    Uart5Tx = 6,
    Uart5Rx = 7,
    Uart7Tx = 8,
    Uart7Rx = 9,
    Uart2Tx = 66,
    Uart2Rx = 67,
    Uart4Tx = 68,
    Uart4Rx = 69,
    Uart6Tx = 70,
    Uart6Rx = 71,
    Uart8Tx = 72,
    Uart8Rx = 73,
}

type DMAChannel = u32;

fn get_addr(channel: DMAChannel) -> u32 {
    return addrs::DMAMUX + (channel * 4);
}

pub fn dma_enable(channel: DMAChannel) {
    let addr = get_addr(channel);
    assign(addr, read_word(addr) | (0x1 << 31));
}

pub fn dma_disable(channel: DMAChannel) {
    let addr = get_addr(channel);
    assign(addr, read_word(addr) & !(0x1 << 31));
}

pub fn dma_trigger_enable(channel: DMAChannel) {
    let addr = get_addr(channel);
    assign(addr, read_word(addr) | (0x1 << 30));
}

pub fn dma_trigger_disable(channel: DMAChannel) {
    let addr = get_addr(channel);
    assign(addr, read_word(addr) & !(0x1 << 30));
}

pub fn dma_configure_source(channel: DMAChannel, source: DMASource) {
    let addr = get_addr(channel);
    assign(addr, read_word(addr) & !(0x3F) | (source as u32));
}

pub fn dma_destination(channel: DMAChannel, destination: u32) {
    let addr = addrs::DMA + 0x1010 + (channel * 0x20);
    assign(addr, destination);
}