use core::arch::asm;

use crate::wait_ns;
use crate::phys::{ Dir };
use crate::phys::gpio::{
    Pin,
    MuxSpeed,
    gpio_speed,
    gpio_direction,
    gpio_set,
    gpio_clear,

};

const GpioSpeed: MuxSpeed = MuxSpeed::Fast;
const GpioPin: Pin = Pin::Gpio7;
const GpioBit: u32 = 0xFFFF_FFFF;

fn on() {
    gpio_set(GpioPin, GpioBit);
    wait_ns(800);
    gpio_clear(GpioPin, GpioBit);
    wait_ns(450);
}

fn off() {
    gpio_set(GpioPin, GpioBit);
    wait_ns(400);
    gpio_clear(GpioPin, GpioBit);
    wait_ns(850);

}

pub fn ws2812_init() {
    gpio_speed(GpioPin, MuxSpeed::Fast);
    gpio_direction(GpioPin, Dir::Output);
}

static mut c: u64 = 500;

#[no_mangle]
pub fn ws2812_loop() {
    
    let mut r = 0;
    while r < 100 {
        off();off();off();off();off();off();off();off();
        on();off();off();on();off();on();on();off();
        off();off();off();off();off();off();off();off();
        r = r + 1;
    }
    
    
    gpio_clear(GpioPin, GpioBit);
    unsafe {
        wait_ns(50000);
        // c = c + 1;
    }
}