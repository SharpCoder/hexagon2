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
const GpioBit: u32 = (0x1 << 3);

fn on() {
    gpio_set(GpioPin, GpioBit);
    wait_ns(700);
    gpio_clear(GpioPin, GpioBit);
    wait_ns(600);
}

fn off() {
    gpio_set(GpioPin, GpioBit);
    wait_ns(350);
    gpio_clear(GpioPin, GpioBit);
    wait_ns(800);

}

pub fn ws2812_init() {
    gpio_speed(GpioPin, MuxSpeed::Fast);
    gpio_direction(GpioPin, Dir::Output);
}

#[no_mangle]
pub fn ws2812_loop() {
    
    
    let mut r = 0;
    while r < 1 {
        on();off();off();on();off();on();on();off();
        off();off();off();off();off();off();off();off();
        off();off();off();off();off();off();off();off();
    }
    
    
    gpio_clear(GpioPin, GpioBit);
    wait_ns(55_000);
}