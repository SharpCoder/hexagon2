use teensycore::debug::debug_str;
use teensycore::math::rand;
use teensycore::system::str::*;
use teensycore::system::vector::*;

/// A ShaderConfig defines the configuration for a particular
/// shader.
#[derive(Copy, Clone)]
pub struct ShaderConfig {
    /// Time (in unix epoch seconds) at which this rule is active
    pub time_range_start: u64,
    /// Time (in unix epoch seconds) at which this rule ends
    pub time_range_end: u64,
    /// The shader which this rule pertains to
    pub shader: Str,
    /// The probability of selection (Between 0 - 255)
    pub probability: u64,
}

#[derive(Copy, Clone)]
pub struct ShaderConfigList {
    pub configs: Vector<ShaderConfig>,
}

impl ShaderConfigList {
    pub fn new() -> Self {
        return ShaderConfigList {
            configs: Vector::new(),
        };
    }

    pub fn size(&self) -> usize {
        return self.configs.size();
    }

    pub fn add_config(&mut self, config: ShaderConfig) {
        self.configs.push(config);
    }

    pub fn get_shader(&self, date: u64) -> Str {
        let mut total_probabilities = 0;
        let mut candidates = Vector::new();

        for config in self.configs.into_iter() {
            if config.time_range_start < date && config.time_range_end > date {
                // Check for immediate winner
                if config.probability == 255 {
                    candidates.free();
                    return config.shader;
                } else {
                    total_probabilities += config.probability;
                    candidates.push(config.clone());
                }
            }
        }

        // Identify target probability
        let mut shuffled = candidates.shuffle();
        candidates.free();

        let target = rand() % total_probabilities;
        let mut accumulator = 0u64;
        for candidate in shuffled.into_iter() {
            accumulator += candidate.probability as u64;
            if accumulator >= target {
                shuffled.free();
                return candidate.shader;
            }
        }

        shuffled.free();
        return Str::new();
    }
}