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
    // attach_irq(Irq::PIT, handle_pit_irq);
    fill_irq(handle_pit_irq);

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
    pit_restart(&PeriodicTimerSource::Timer0);






    // let clk_source = TimerSource::GPT1;

    // // Enable Oscillator
    // timer::timer_disable(&clk_source);
    // timer::timer_disable_irq(&clk_source);
    // timer::timer_clear_status(&clk_source);
    // timer::timer_assert_reset(&clk_source);
    // timer::timer_set_clock(&clk_source, TimerClock::Peripheral);
    // timer::timer_set_compare_value(&clk_source, 0x0000_0001);
    // timer::timer_enable(&clk_source);

    // attach_irq(Irq::GPT1, handle_clock_irq);  
    // enable_irq(Irq::GPT1);

    // timer::timer_enable_irq(&clk_source);
}

fn handle_pit_irq() {
    
    unsafe {
        clock_counter += 31;
        clock_decimal_counter += 1;

        if clock_decimal_counter > 100 {
            clock_counter += 50;
            clock_decimal_counter = 0;
        }
    }

    pit_clear_interrupts(&PeriodicTimerSource::Timer0);
    crate::dsb();
}

// fn handle_clock_irq() {

//     // crate::debug_blink(10);

//     unsafe {
//         clock_counter += 20;
//         clock_decimal_counter += 1;

//         if clock_decimal_counter >= 1000 {
//             clock_decimal_counter = 0;
//             clock_counter += 0;
//         }
//     }


//     timer::timer_clear_status(&TimerSource::GPT1);
//     crate::dsb();

//     // irq_clear_pending();
//     // pit_clear_interrupts(&PeriodicTimerSource::Timer1);
// }

pub fn nanos() -> u64 {
    unsafe {
        return clock_counter;
    }
}