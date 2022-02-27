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
}

impl ThermalTask {
    pub fn new(driver: Max31820Driver) -> Self {
        return ThermalTask {
            driver: driver,
            next_event: 0,
        };
    }

    pub fn init(&self) {
        let mut prng_seed = 1337;
        let primes = [3,5,7,11,13,17,19,23,29];
        let samples = [
            self.driver.read_temperature(),
            self.driver.read_temperature(),
            self.driver.read_temperature(),
            self.driver.read_temperature(),
            self.driver.read_temperature(),
        ];

        for i in 0 .. samples.len() {
            if samples[i].is_some() {
                prng_seed += samples[i].unwrap() as u64 * primes[i];
            }
        }
        
        debug_u64(prng_seed, b"prng seed");
        seed_rand(prng_seed);
    }
}