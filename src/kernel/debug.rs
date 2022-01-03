use core::arch::asm;
use crate::phys::pins::*;
use crate::kernel::math::*;
use crate::kernel::serio::*;

#[derive(Copy, Clone)]
pub struct BlinkConfig {
    pub speed: Speed,
    pub remaining_count: u8,
}

#[derive(Copy, Clone)]
pub enum Speed {
    Slow = (crate::MS_TO_NANO * 1100u64) as isize,
    Fast = (crate::MS_TO_NANO * 350u64) as isize,
    Normal = (crate::MS_TO_NANO * 700u64) as isize,
}

pub static mut BLINK_CONFIG: BlinkConfig = BlinkConfig {
    speed: Speed::Normal,
    remaining_count: 0,
};

pub fn blink_led_on() {
    pin_out(13, Power::High);
}

pub fn blink_led_off() {
    pin_out(13, Power::Low);
}

pub fn blink(count: u8, speed: Speed) {
    unsafe {
        if BLINK_CONFIG.remaining_count == 0 {
            BLINK_CONFIG.speed = speed;
            BLINK_CONFIG.remaining_count = count;
        }
    }
}

pub fn debug_hex(hex: u32, message: &[u8]) {
    serio_write(b"0x");
    serio_write(&to_base(hex, 16));
    debug_str(message);
}

pub fn debug_u64(val: u64, message: &[u8]) {
    serio_write(&itoa_u64(val));
    debug_str(message);
}

pub fn debug_u32(val: u32, message: &[u8]) {
    serio_write(&to_base(val, 10));
    debug_str(message);
}

pub fn debug_str(message: &[u8]) {
    serio_write(message);
    serio_write(b"\n");
}