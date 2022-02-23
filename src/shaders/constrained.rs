use crate::shaders::core::*;
use teensycore::math::*;

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

    fn randomize(&self, context: ShaderContext) -> ShaderContext {
        let mut next_context = context;
        let start = (teensycore::math::rand() % 254) as i32;
        let end = (teensycore::math::rand() % 254) as i32;

        next_context.registers[1] = min(start, end);
        next_context.registers[2] = max(end, start);
        next_context.registers[3] = 1;
        next_context.registers[0] = next_context.registers[1] + (teensycore::math::rand() % (next_context.registers[2] as u64 - next_context.registers[1] as u64)) as i32;
        return next_context;
    }
}