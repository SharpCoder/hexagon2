use crate::math::*;
use crate::phys::pins::*;
use crate::serio::*;

#[derive(Copy, Clone)]
pub struct BlinkConfig {
    pub speed: Speed,
    pub remaining_count: u8,
}

#[derive(Copy, Clone)]
pub enum Speed {
    Slow = (crate::MS_TO_NANO * 1000u64) as isize,
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
        if BLINK_CONFIG.remaining_count == 0 || speed as isize != BLINK_CONFIG.speed as isize {
            BLINK_CONFIG.speed = speed;
            BLINK_CONFIG.remaining_count = count;
        }
    }
}

// Silly function to skip repeated spaces which
// are common int he iota function I wrote.
fn send_with_trim(message: &[u8]) {
    for i in 0 .. message.len() {
        let cur = message[i];
        if i < message.len() - 1 && message[i+1] == b' ' && cur == b' ' {
            continue;
        } else {
            serio_write(&[cur]);
        }
    }
}

pub fn debug_hex(hex: u32, message: &[u8]) {
    serio_write(b"0x");
    send_with_trim(&to_base(hex as u64, 16));
    debug_str(message);
}

pub fn debug_u64(val: u64, message: &[u8]) {
    send_with_trim(&itoa_u64(val));
    debug_str(message);
}

pub fn debug_u32(val: u32, message: &[u8]) {
    send_with_trim(&to_base(val as u64, 10));
    debug_str(message);
}

pub fn debug_str(message: &[u8]) {
    serio_write(message);
    serio_write(b"\n");
}