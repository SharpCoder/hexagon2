/**
 *  This is a system device which keeps track of time by using the periodic timer 
 **/

use core::arch::asm;
use crate::phys::{
    addrs,
    assign,
    read_word,
};
use crate::phys::timer;
use crate::phys::timer::{
    TimerClock,
    TimerSource,
};
use crate::phys::periodic_timers::*;
use crate::phys::gpio::{
    Pin,
    gpio_set,
    gpio_clear,
};
use crate::phys::irq::*;

static mut clock_counter: u64 = 0;
static mut clock_decimal_counter: u64 = 0;

pub fn clock_init() {
    // // Undo clock gating
    assign(addrs::CCM_CCGR1, read_word(addrs::CCM_CCGR1) | (0x3 << 12));
    
    // Select 150MHz clock
    assign(addrs::CCM_CSCMR1, read_word(addrs::CCM_CSCMR1) & !(0x1 << 6));

    // Attach the interrupts
    attach_irq(Irq::PIT, handle_pit_irq);
    // fill_irq(handle_pit_irq);

    // Set CTRL 0
    pit_configure(&PeriodicTimerSource::Timer0, PITConfig {
        chained: false,
        irq_en: false,
        en: false,
    });

    // Clear interrupts
    pit_clear_interrupts(&PeriodicTimerSource::Timer0);

    // Load
    pit_load_value(&PeriodicTimerSource::Timer0, 0x0000_0014);

    // Enable interrupts
    pit_configure(&PeriodicTimerSource::Timer0, PITConfig {
        chained: false,
        irq_en: true,
        en: true,
    });
    

    enable_irq(Irq::PIT);

    // Secret sauce which makes it all work otherwise you are bound
    // to a default timeout that takes like a minute.
    pit_restart(&PeriodicTimerSource::Timer0);
}

fn handle_pit_irq() {
    
    // The neopixel timing has proven this is not the actual clock speed.
    // But it's pretty close.
    unsafe {
        clock_counter += 31;
        clock_decimal_counter += 1;

        if clock_decimal_counter > 10 {
            clock_counter += 5;
            clock_decimal_counter = 0;
        }
    }

    pit_clear_interrupts(&PeriodicTimerSource::Timer0);
    crate::dsb();
}

pub fn nanos() -> u64 {
    unsafe {
        return clock_counter;
    }
}