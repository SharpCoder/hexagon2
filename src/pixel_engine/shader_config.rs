use teensycore::math::rand;
use teensycore::system::str::*;
use teensycore::system::vector::*;

fn shader_to_str(shader: [u8; 32]) -> Str {
    let mut result = Str::new();
    for char in shader {
        if char == 0 {
            break;
        } else {
            result.append(&[char]);
        }
    }
    return result;
}

/// A ShaderConfig defines the configuration for a particular
/// shader.
#[derive(Copy, Clone)]
pub struct ShaderConfig {
    /// Time (in unix epoch seconds) at which this rule is active
    pub time_range_start: u64,
    /// Time (in unix epoch seconds) at which this rule ends
    pub time_range_end: u64,
    /// The shader which this rule pertains to
    pub shader: [u8; 32],
    /// The probability of selection (Between 0 - 255)
    pub probability: u8,
}

pub struct ShaderConfigList {
    configs: Vector<ShaderConfig>,
}

impl ShaderConfigList {
    pub fn new() -> Self {
        return ShaderConfigList {
            configs: Vector::new(),
        };
    }

    pub fn add_config(&mut self, config: ShaderConfig) {
        self.configs.push(config);
    }

    pub fn get_shader(&self, date: u64) -> Str {
        let mut total_probabilities = 0;
        let mut candidates = Vector::new();

        for config in self.configs.into_iter() {
            if config.time_range_start > date && config.time_range_end < date {
                // Check for immediate winner
                if config.probability == 255 {
                    candidates.free();
                    return shader_to_str(config.shader);
                } else {
                    total_probabilities += config.probability as u64;
                    candidates.push(config);
                }
            }
        }

        // Identify target probability
        let target = rand() % total_probabilities;
        let mut accumulator = 0u64;
        for candidate in candidates.into_iter() {
            accumulator += candidate.probability as u64;
            if accumulator >= target {
                candidates.free();
                return shader_to_str(candidate.shader);
            }
        }

        candidates.free();
        return Str::new();
    }
}