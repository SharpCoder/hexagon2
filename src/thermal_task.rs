use teensycore::*;
use teensycore::clock::*;
use teensycore::debug::*;
use teensycore::math::seed_rand;
use teensycore::phys::irq::disable_interrupts;
use teensycore::phys::irq::enable_interrupts;

use crate::drivers::max31820::Max31820Driver;

/// This task is responsible for sampling the ambiant temperature
/// and keeping the system informed of changes.

pub struct ThermalTask {
    driver: Max31820Driver,
    next_event: u64,
    count: usize,
    samples: [Option<u16>; 5],
    pub loaded: bool,
}

impl ThermalTask {
    pub fn new(driver: Max31820Driver) -> Self {
        return ThermalTask {
            driver: driver,
            next_event: 0,
            count: 0,
            loaded: false,
            samples: [None; 5],
        };
    }

    pub fn init(&self) {
        let samples = [
            self.driver.read_temperature(),
            self.driver.read_temperature(),
            self.driver.read_temperature(),
            self.driver.read_temperature(),
            self.driver.read_temperature(),
        ];

    }

    pub fn system_loop(&mut self) {
        if self.count < 5 {
            self.samples[self.count] = self.driver.read_temperature();
            self.count += 1;
        } else if self.count == 5 {
            let mut prng_seed = 1337;
            let primes = [3,5,7,11,13,17,19,23,29];

            for i in 0 .. self.samples.len() {
                if self.samples[i].is_some() {
                    prng_seed += self.samples[i].unwrap() as u64 * primes[i];
                }
            }
            
            debug_u64(prng_seed, b"prng seed");
            seed_rand(prng_seed);

            self.loaded = true;
            self.count += 1;
        } else {

        }
    }
}