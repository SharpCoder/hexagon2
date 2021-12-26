use core::arch::asm;

use crate::phys::{ Dir };
use crate::phys::gpio::{
    Pin,
    MuxSpeed,
    gpio_speed,
    gpio_direction,
    gpio_set,
    gpio_clear,

};

const WaitMod: u32 = 4;
const GpioSpeed: MuxSpeed = MuxSpeed::Fast;
const GpioPin: Pin = Pin::Gpio7;
const GpioBit: u32 = 0x1 << 3;

// Note: GRB is the sequence
struct LightNode {
    green: u8,
    red: u8,
    blue: u8,
}

struct WS2812 <'a> {
    nodes: &'a [LightNode],
}

fn on() {
    gpio_set(GpioPin, GpioBit);
    wait(WaitMod, 40);
    gpio_clear(GpioPin, GpioBit);
    wait(WaitMod, 22);
}

fn off() {
    gpio_set(GpioPin, GpioBit);
    wait(WaitMod, 20);
    gpio_clear(GpioPin, GpioBit);
    wait(WaitMod, 42);

}

fn wait(ms: u32, modifier: u32) {
    let mut r = 0;
    let target = ms * modifier;
    while r < target {
        r = r + 1;
        unsafe {
            asm!("nop");
        }
    }
}

pub fn ws2812_init() {
    gpio_speed(GpioPin, MuxSpeed::Fast);
    gpio_direction(GpioPin, Dir::Output);
}

pub fn ws2812_loop() {
    let node = LightNode {
        green: 0xFF,
        red: 0x00,
        blue: 0x00,
    };
    

    on();
    off();
    off();
    on();
    off();
    on();
    on();
    off();
    off();off();off();off();off();off();off();off();
    off();off();off();off();off();off();off();off();

    off();
    wait(200000, 1);
        

}