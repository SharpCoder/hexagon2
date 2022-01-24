use crate::drivers::ws2812::*;

#[derive(Copy, Clone)]
pub struct ShaderContext {
    pub node_id: usize,
    pub total_nodes: usize,
    pub temperature: i32,
    pub audio_bands: [u32; 7], // Data from the band equalizer
    pub registers: [i32; 10],
    pub color: u32,
}

impl ShaderContext {
    pub const fn new(id: usize, total_nodes: usize) -> Self {
        return ShaderContext {
            node_id: id,
            total_nodes: total_nodes,
            temperature: 0,
            audio_bands: [0; 7],
            registers: [0; 10],
            color: 0xFF0000,
        }
    }
}

pub trait Shader<const SIZE: usize> {
    fn name(&self) -> &[u8];
    fn init(&mut self, context: ShaderContext) -> ShaderContext;
    fn update(&mut self, context: ShaderContext) -> ShaderContext;
}

/**
Basic rainbow shader
*/
pub struct BasicShader {}
impl <const SIZE: usize> Shader<SIZE> for BasicShader {
    fn name(&self) -> &[u8] { return b"Basic"; }

    fn init(&mut self, context: ShaderContext) -> ShaderContext {
        let mut next_context = context;
        next_context.registers[0] = (255i32 / context.total_nodes as i32) * context.node_id as i32;
        return next_context;
    }
    fn update(&mut self, context: ShaderContext) -> ShaderContext {
        let mut next_context: ShaderContext = context;
        let count = context.registers[0] as u8;

        next_context.color = wheel(count);
        next_context.registers[0] += 1;
        if next_context.registers[0] > 255 {
            next_context.registers[0] = 0;
        }
        return next_context;
    }
}

impl BasicShader {
    pub const fn new() -> BasicShader {
        return BasicShader {};
    }
}


/**
Xmas themed shader
*/
pub struct XmasShader {}
impl XmasShader {
    pub const fn new() -> XmasShader {
        return XmasShader {};
    }
}
impl <const SIZE: usize> Shader<SIZE> for XmasShader {
    fn name(&self) -> &[u8] { return b"Xmas"; }

    fn init(&mut self, context: ShaderContext) -> ShaderContext {
        let mut next_context = context;
        // Randomize the starting position for each node
        next_context.registers[0] = (teensycore::math::rand() % 170) as i32;
        return next_context;
    }
    
    fn update(&mut self, context: ShaderContext) -> ShaderContext {
        let mut next_context: ShaderContext = context;
        let pos = context.registers[0] as u8;

        // R -> G -> R wheel
        if pos < 85 {
            next_context.color = rgb_to_hex(255 - pos * 3, pos * 3, 0);
        } else if pos < 170 {
            next_context.color = rgb_to_hex((pos - 85) * 3, 255 - (pos - 85) * 3, 0);
        } else {
            next_context.registers[0] = 0;
            next_context.color = rgb_to_hex(252, 3, 0);
        }

        next_context.registers[0] += 1;
        return next_context;
    }
}


/**
One of my favorites, constrained rainbow
*/
pub struct ConstrainedRainbowShader {}
impl ConstrainedRainbowShader {
    pub const fn new() -> ConstrainedRainbowShader {
        return ConstrainedRainbowShader {};
    }
}
impl <const SIZE: usize> Shader<SIZE> for ConstrainedRainbowShader {
    fn name(&self) -> &[u8] { return b"Constrained Rainbow"; }

    fn init(&mut self, context: ShaderContext) -> ShaderContext {
        let mut next_context = context;
        
        // let start = 0u64;
        // let end = 45u64;
        let start = 0u64;
        let end = 78u64;

        // Registers 1 and 2 are lower and upper limit
        next_context.registers[1] = start as i32;
        next_context.registers[2] = end as i32;
        // Register 3 is whether to go up or down
        next_context.registers[3] = 1;
        // Randomize the starting position for each node
        next_context.registers[0] = (start + teensycore::math::rand() % (end - start)) as i32;
        return next_context;
    }
    
    fn update(&mut self, context: ShaderContext) -> ShaderContext {
        let mut next_context: ShaderContext = context;
        let pos = context.registers[0] as u8;
        let next_pos: i32;

        if context.registers[3] > 0 {
            next_pos = pos as i32 + 1;
        } else {
            next_pos = pos as i32 - 1;
        }

        if next_pos >= context.registers[2] {
            next_context.registers[3] = 0;
        } else if next_pos <= context.registers[1] {
            next_context.registers[3] = 1;
        }

        next_context.color = wheel(pos as u8);
        next_context.registers[0] = next_pos as i32;
        return next_context;
    }
}


/* 
Audio Equalizer uses the onboard equalizer chip attuned to
the speaker in order to create an effect that mimics
the sound around it.
*/
pub struct AudioEqualizerShader {}
impl AudioEqualizerShader {
    pub const fn new() -> AudioEqualizerShader {
        return AudioEqualizerShader {};
    }
}
impl <const SIZE: usize> Shader<SIZE> for AudioEqualizerShader {
    fn name(&self) -> &[u8] { return b"Audio Equalizer"; }

    fn init(&mut self, context: ShaderContext) -> ShaderContext {
        let next_context = context;
        return next_context;
    }
    
    fn update(&mut self, context: ShaderContext) -> ShaderContext {
        let next_context: ShaderContext = context;
        return next_context;
    }
}
