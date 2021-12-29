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

const GPIO_SPEED: MuxSpeed = MuxSpeed::Fast;
const GPIO_PIN: Pin = Pin::Gpio7;
const GPIO_BIT: u32 = 0x1 << 3;

fn on() {
    gpio_set(&GPIO_PIN, GPIO_BIT);
    wait_ns(700);
    gpio_clear(&GPIO_PIN, GPIO_BIT);
    wait_ns(600);
}

fn off() {
    gpio_set(&GPIO_PIN, GPIO_BIT);
    wait_ns(350);
    gpio_clear(&GPIO_PIN, GPIO_BIT);
    wait_ns(800);

}

pub fn ws2812_init() {
    gpio_speed(&GPIO_PIN, GPIO_SPEED);
    gpio_direction(&GPIO_PIN, Dir::Output);
}

#[no_mangle]
pub fn ws2812_loop() {
    
    
    let mut r = 0;
    while r < 1 {
        on();off();off();on();off();on();on();off();
        off();off();off();off();off();off();off();off();
        off();off();off();off();off();off();off();off();
        r += 1;
    }
    
    
    gpio_clear(&GPIO_PIN, GPIO_BIT);
    wait_ns(55_000);
}