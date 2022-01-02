use core::arch::asm;
use crate::phys::pins::*;
use crate::kernel::math::*;
use crate::kernel::serio::*;

#[derive(Copy, Clone)]
pub enum Speed {
    Slow = 259900000,
    Fast = 20000000,
    Normal = 159900000,
}

fn wait(speed: &Speed) {
    let mut r: u32 = 0;
    while r < (*speed as u32) {
        r += 1;
        unsafe {
            asm!("nop");
        }
    }
}

pub fn blink(count: u32, speed: Speed) {
    let mut i = 0;
    while i < count {
        pin_out(13, Power::High);
        wait(&speed);
        pin_out(13, Power::Low);
        wait(&speed);
        i += 1;
    }
}

#[no_mangle]
pub fn debug_u32_asm(hex: u32) {
        serio_write(b"0x");
        serio_write(&to_base(hex, 16));
        serio_write(b"\n");
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