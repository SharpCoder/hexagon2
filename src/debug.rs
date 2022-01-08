use crate::{math::*, MS_TO_NANO};
use crate::phys::pins::*;
use crate::serio::*;
use crate::*;

pub static DEBUG_UART_DEVICE: SerioDevice = SerioDevice::Uart4;

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
            BLINK_CONFIG.remaining_count = count * 2; // Multiply by two, one for each blink state
        }
    }
}

pub fn blink_accumulate() {
    unsafe {
        BLINK_CONFIG.speed = Speed::Fast;
        BLINK_CONFIG.remaining_count += 2;
    }
}

/***
 * This method will flash LED 13
 * using hardware-level waits
 * (hard wait) instead of relying
 * on gates.
 * */
pub fn blink_hardware(count: u8) {
    for _ in 0 .. count {
        blink_led_on();
        wait_ns(MS_TO_NANO * 250);
        blink_led_off();
        wait_ns(MS_TO_NANO * 150);
    }
}

pub fn debug_hex(hex: u32, message: &[u8]) {
    serial_write(DEBUG_UART_DEVICE, b"0x");
    serial_write_vec(DEBUG_UART_DEVICE, to_base(hex as u64, 16));
    serial_write(DEBUG_UART_DEVICE, b" ");
    debug_str(message);
}

pub fn debug_u64(val: u64, message: &[u8]) {
    serial_write_vec(DEBUG_UART_DEVICE, itoa_u64(val));
    serial_write(DEBUG_UART_DEVICE, b" ");
    debug_str(message);
}

pub fn debug_u32(val: u32, message: &[u8]) {
    serial_write_vec(DEBUG_UART_DEVICE, to_base(val as u64, 10));
    serial_write(DEBUG_UART_DEVICE, b" ");
    debug_str(message);
}

pub fn debug_str(message: &[u8]) {
    serial_write(DEBUG_UART_DEVICE, message);
    serial_write(DEBUG_UART_DEVICE, b"\n");
}