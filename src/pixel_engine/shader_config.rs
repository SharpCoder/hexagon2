use teensycore::system::str::*;

/// A ShaderConfig defines the configuration for a particular
/// shader.
pub struct ShaderConfig <'a> {
    /// Time (in unix epoch seconds) at which this rule is active
    pub time_range_start: u64,
    /// Time (in unix epoch seconds) at which this rule ends
    pub time_range_end: u64,
    /// The shader which this rule pertains to
    pub shader: &'a [u8],
    /// The probability of selection (Between 0 - 255)
    pub probability: u8,
}

impl <'a> ShaderConfig<'a> {
    
}