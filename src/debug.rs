use core::arch::asm;
use crate::phys::pins::*;

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
