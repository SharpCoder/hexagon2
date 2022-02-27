use crate::shaders::core::*;

/**
4th of july themed shader
*/
pub struct IndependenceShader {}
impl IndependenceShader {
    pub const fn new() -> IndependenceShader {
        return IndependenceShader {};
    }
}
impl <const SIZE: usize> Shader<SIZE> for IndependenceShader {
    fn name(&self) -> &[u8] { return b"Independence"; }

    fn init(&mut self, context: ShaderContext) -> ShaderContext {
        let mut next_context = context;
        // Randomize the starting position for each node
        next_context.registers[0] = (255i32 / context.total_nodes as i32) * context.node_id as i32;
        // next_context.registers[0] = (teensycore::math::rand() % 170) as i32;
        return next_context;
    }
    
    fn update(&mut self, context: ShaderContext) -> ShaderContext {
        let mut next_context: ShaderContext = context;
        let pos = context.registers[0] as u8;

        // R -> B -> W wheel
        if pos < 85 {
            // Red -> Blue
            next_context.color = rgb_to_hex(
                (255 - pos * 3), 
                0, 
                (pos * 3)
            );

        } else if pos < 170 {
            // Blue -> White
            next_context.color = rgb_to_hex(
                (pos - 85) * 3, 
                (pos - 85) * 3, 
                255,
            );
        } else if pos < 255 {
            // White -> Red
            next_context.color = rgb_to_hex(
                255, 
                255 - (pos - 170) * 3, 
                255 - (pos - 170) * 3,
            );
        } else {
            next_context.registers[0] = 0;
            next_context.color = rgb_to_hex(255, 0, 0);
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
