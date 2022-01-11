use crate::phys::periodic_timers::pit_clear_interrupts;
use crate::wait_ns;
use crate::irq::*;
use crate::phys::pins::*;
use crate::debug::*;
use crate::clock::*;
use crate::system::vector::*;
use crate::system::map::*;
use crate::phys::periodic_timers::*;
use core::arch::asm;

enum Stage {
    First,
    Second,
}

pub struct WS2812V2Driver {
    leds: [u32; 180],
    index: usize,
    pin: usize,
    bit: usize,
    stage: Stage,
    next_event: u64, // nanoseconds
    rest: bool,
    flush: bool,
    size: usize,
}

static mut WS2812_DRIVER_INSTANCES: WS2812V2Driver = WS2812V2Driver {
    leds: [0;180],
    index: 0,
    pin: 18,
    bit: 23,
    stage: Stage::First,
    next_event: 0,
    rest: false,
    flush: false,
    size: 1,
};

impl WS2812V2Driver {

    pub fn new(pin: usize, led_count: usize) {
        // Configure the pin
        pin_mode(pin, Mode::Output);
        pin_pad_config(pin, PadConfig {
            hysterisis: true,               // HYS
            resistance: PullUpDown::PullDown100k, // PUS
            pull_keep: PullKeep::Pull,            // PUE
            pull_keep_en: true,             // PKE
            open_drain: false,               // ODE
            speed: PinSpeed::Fast150MHz,                // SPEED
            drive_strength: DriveStrength::Disabled,  // DSE
            fast_slew_rate: true,           // SRE
        });

        pin_out(pin, Power::Low);
        let mut driver = WS2812V2Driver {
            leds: [0;180],
            index: 0,
            pin: pin,
            bit: 23,
            stage: Stage::First,
            next_event: 0,
            rest: false,
            flush: false,
            size: led_count,
        };

        unsafe {
            // WS2812_DRIVER_INSTANCES = driver;
        }

        // Setup periodic timer
        // pit_enable_clock();
        disable_interrupts();
        pit_configure(&PeriodicTimerSource::Timer3, PITConfig { 
            chained: false, 
            irq_en: true, 
            en: false 
        });
        pit_load_value(&PeriodicTimerSource::Timer3, 0x3F);
        
        irq_attach(Irq::PeriodicTimer, ws2812_handle_irq);
        irq_enable(Irq::PeriodicTimer);
        pit_restart(&PeriodicTimerSource::Timer3);
        irq_priority(Irq::PeriodicTimer, 1);
        enable_interrupts();

        // return driver;
    }

    pub fn get_instance() -> &'static mut Self {
        return unsafe { &mut WS2812_DRIVER_INSTANCES };
    }

    pub fn set_color(&mut self, index: usize, rgb: u32) {
        if index >= self.size {
            return;
        }

        self.leds[index] = rgb;
    }

    pub fn flush(&mut self) {
        unsafe { &mut WS2812_DRIVER_INSTANCES }.flush = true;
        // self.flush = true;
    }

    pub fn process(&mut self) {
        if !self.flush {
            return;
        }

        // blink_accumulate();
        if crate::clock::nanos() < self.next_event  {
            // debug_str(b"Here");
            unsafe {
                asm!("nop");
            }
            return;
        }

        // debug_u64(nanos() - self.next_event, b"elapsed");
        // debug_u32(self.bit as u32, b"bits");
        // debug_u64(nanos() - self.next_event, b"elapsed time");

        // The gate is open, process currenet command
        if self.rest {
            pin_out(self.pin, Power::Low);
        } else {
            match self.stage {
                Stage::First => {
                    pin_out(self.pin, Power::High);
                },
                Stage::Second => {
                    pin_out(self.pin, Power::Low);
                }
            }
        }

        let origin = nanos();

        // Determine the timing
        let color = self.leds.get(self.index).unwrap();
        let signal_high = ((0x1 << self.bit) & color) > 0;

        // Calculate how long to leave the state active
        let time: u64;

        if self.rest {
            time = 85_000;
        } else if signal_high {
            time = match self.stage {
                Stage::First => 700,
                Stage::Second => 600, 
            };
        } else {
            time = match self.stage {
                Stage::First => 350,
                Stage::Second => 800,
            };
        }


        // Set the next event
        // debug_u64(time,b"wait time");
        self.next_event = origin + time;
        self.advance();
    }

    /// Advance the bit counter
    fn advance(&mut self) {
        if self.rest {
            self.rest = false;
            self.flush = false;
            return;
        }

        match self.stage {
            Stage::First => {
                self.stage = Stage::Second;
                return;
            },
            Stage::Second => {
                self.stage = Stage::First;
            }
        }

        if self.bit == 0 {
            self.index += 1;
            self.bit = 23;
        } else {
            self.bit -= 1;
        }

        if self.index > self.size {
            self.rest = true;
            self.index = 0;
            self.bit = 23;
            self.stage = Stage::First;
        }
    }
}

fn ws2812_handle_irq() {
    disable_interrupts();
    unsafe {
        (&mut WS2812_DRIVER_INSTANCES).process();
    }
    pit_clear_interrupts(&PeriodicTimerSource::Timer3);
    enable_interrupts();
}