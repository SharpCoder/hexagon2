use core::arch::asm;
use crate::phys::gpio::*;

#[derive(Copy, Clone)]
pub enum Speed {
    Slow = 100000000,
    Fast = 10000000,
    Normal = 90000000,
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
        gpio_set(Pin::Gpio7, 0x1 << 3);
        wait(&speed);
        gpio_clear(Pin::Gpio7, 0x1 << 3);
        wait(&speed);
        i += 1;
    }
}