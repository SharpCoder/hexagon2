pub mod addrs;
pub mod dma;
pub mod gpio;
pub mod irq;
pub mod periodic_timers;
pub mod timer;
pub mod uart;

pub enum Dir {
    Input,
    Output,
}

pub fn write_byte(address: u32, value: u8) {
    unsafe {
        *(address as *mut u8) = value;
    }
}

pub fn assign(address: u32, value: u32) {
    unsafe {
        *(address as *mut u32) = value;
    }
}

pub fn read_word(address: u32) -> u32 {
    unsafe {
        return *(address as *mut u32);
    }
}

pub fn clear_bit(number: u32, bit: u8) -> u32 {
    return number & !(0x01 << bit);
}

pub fn set_bit(number: u32, bit: u8) -> u32 {
    return number | (0x01 << bit);
}
