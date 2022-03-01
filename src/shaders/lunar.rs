use crate::shaders::core::*;
use teensycore::math::max;

pub struct LunarShader {}
impl LunarShader {
    pub const fn new() -> LunarShader {
        return LunarShader {};
    }
}
impl <const SIZE: usize> Shader<SIZE> for LunarShader {
    fn name(&self) -> &[u8] { return b"Lunar New Year"; }

    fn init(&mut self, context: ShaderContext) -> ShaderContext {
        let mut next_context = context;
        // Randomize the starting position for each node
        next_context.registers[0] = (255i32 / context.total_nodes as i32) * context.node_id as i32;
        // next_context.registers[0] = (teensycore::math::rand() % 170) as i32;
        return next_context;
    }
    
    fn update(&mut self, context: ShaderContext) -> ShaderContext {
        let mut next_context: ShaderContext = context;
        let pos = context.registers[0] as u16;

        // Dark R -> R -> Y -> Dark R wheel
        if pos < 56 {
            next_context.color = rgb_to_hex(
                199 + pos as u8, 
                7, 
                2
            );
        } else if pos < 133 {
            // Red -> Yellow
            next_context.color = rgb_to_hex(
                255, 
                7 + (pos - 56) as u8, 
                2,
            );
        } else if pos < 210 {
            // White -> Red
            next_context.color = rgb_to_hex(
                max(255 - (pos - 133) as u8, 199), 
                max(84 - (pos - 133) as u8, 7), 
                2,
            );
        } else {
            next_context.registers[0] = 0;
            next_context.color = rgb_to_hex(199, 7, 2);
        }

        next_context.registers[0] += 1;
        return next_context;
    }

    fn randomize(&self, context: ShaderContext) -> ShaderContext {
        let mut next_context = context;
        next_context.registers[0] = (teensycore::math::rand() % 170) as i32;
        return next_context;
    }
}
