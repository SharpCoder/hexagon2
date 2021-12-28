/**
 *  This is a system device which keeps track of time by using the periodic timer 
 **/

use crate::phys::{
    addrs,
    assign,
    read_word,
};
use crate::phys::periodic_timers::*;
use crate::phys::irq::*;

static mut CLOCK_COUNTER: u64 = 0;
static mut CLOCK_TENS_COUNTER: u64 = 0;

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
    

    irq_enable(Irq::PIT);

    // Secret sauce which makes it all work otherwise you are bound
    // to a default timeout that takes like a minute.
    pit_restart(&PeriodicTimerSource::Timer0);
}

fn handle_pit_irq() {
    
    // The neopixel timing has proven this is not the actual clock speed.
    // But it's pretty close.
    unsafe {
        CLOCK_COUNTER += 31;
        CLOCK_TENS_COUNTER += 1;

        if CLOCK_TENS_COUNTER > 10 {
            CLOCK_COUNTER += 5;
            CLOCK_TENS_COUNTER = 0;
        }
    }

    pit_clear_interrupts(&PeriodicTimerSource::Timer0);
    crate::dsb();
}

pub fn nanos() -> u64 {
    unsafe {
        return CLOCK_COUNTER;
    }
}