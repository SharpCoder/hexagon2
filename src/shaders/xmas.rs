use crate::shaders::core::*;

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
        next_context.registers[0] = (255i32 / context.total_nodes as i32) * context.node_id as i32;
        // next_context.registers[0] = (teensycore::math::rand() % 170) as i32;
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

    fn randomize(&self, context: ShaderContext) -> ShaderContext {
        let mut next_context = context;
        next_context.registers[0] = (teensycore::math::rand() % 170) as i32;
        return next_context;
    }
}
