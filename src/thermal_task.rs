use teensycore::*;
use teensycore::clock::*;
use teensycore::debug::*;
use teensycore::math::seed_rand;

use crate::drivers::max31820::Max31820Driver;

/// This task is responsible for sampling the ambiant temperature
/// and keeping the system informed of changes.
const SAMPLES: usize = match crate::CYCLE_MODE {
    true => 1,
    false => 8,
};

pub struct ThermalTask {
    driver: Max31820Driver,
    next_event: uNano,
    count: usize,
    samples: [Option<u16>; SAMPLES],
    pub loaded: bool,
}

impl ThermalTask {
    pub fn new(driver: Max31820Driver) -> Self {
        return ThermalTask {
            driver: driver,
            next_event: 0,
            count: 0,
            loaded: false,
            samples: [None; SAMPLES],
        };
    }

    pub fn init(&mut self) {
        match self.driver.read_temperature() {
            None => {},
            Some(sample) => {
                seed_rand(sample as u64);

                // If we are using wifi, this is the only sampling we need.
                if crate::USE_WIFI {
                    self.loaded = true;
                }
            }
        }
    }

    pub fn system_loop(&mut self) {
        let time = nanos();
        if !crate::USE_WIFI && !self.loaded && time > self.next_event {
            if self.count < SAMPLES {
                self.samples[self.count] = self.driver.read_temperature();
                self.count += 1;
            } else if self.count == SAMPLES {
                let mut prng_seed = 1337;
                let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];
    
                for i in 0 .. self.samples.len() {
                    if self.samples[i].is_some() {
                        prng_seed += self.samples[i].unwrap() as u64 * primes[i];
                    }
                }
                
                // Reach into the dregs of what we have available and utilize
                // the bits of entropy associated with timing delays aggregated
                // up to this point. (NOTE: This actually makes a significant difference)
                prng_seed *= nanos() as u64;

                debug_u64(prng_seed, b"prng seed");
                seed_rand(prng_seed);
    
                self.loaded = true;
                self.count += 1;
            }
            self.next_event = time + MS_TO_NANO * 7 * 85 * crate::WORLD_MUTIPLIER;
        }
    }
}