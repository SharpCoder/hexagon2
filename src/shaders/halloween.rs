use crate::shaders::core::*;

fn sub_if(val: u8, sub: u8) -> u8 {
    if val > sub {
        return val - sub;
    } else {
        return 0;
    }
}

/**
4th of july themed shader
*/
pub struct HalloweenShader {}
impl HalloweenShader {
    pub const fn new() -> HalloweenShader {
        return HalloweenShader {};
    }
}
impl <const SIZE: usize> Shader<SIZE> for HalloweenShader {
    fn name(&self) -> &[u8] { return b"Halloween"; }

    fn init(&mut self, context: ShaderContext) -> ShaderContext {
        let mut next_context = context;
        // Randomize the starting position for each node
        next_context.registers[0] = (255i32 / context.total_nodes as i32) * context.node_id as i32;
        // next_context.registers[0] = (teensycore::math::rand() % 170) as i32;
        return next_context;
    }
    
    fn update(&mut self, context: ShaderContext) -> ShaderContext {
        let mut next_context: ShaderContext = context;
        let pos = context.registers[0] as u32;

        // R -> B -> W wheel
        if pos < 85 {
            // Black -> Orange
            next_context.color = rgb_to_hex(
                (pos * 3) as u8,
                ((pos * 15) / 20) as u8, 
                0
            );
        } else if pos < 170 {
            // Orange -> Red
            next_context.color = rgb_to_hex(
                255,
                64 - (((pos - 85) * 15) / 20) as u8,
                0
            );
        } else if pos < 255 {
            // Red -> Black
            next_context.color = rgb_to_hex(
                255 - ((pos - 170) * 3) as u8,
                1,
                0
            );
        } else {
            next_context.registers[0] = 0;
            next_context.color = rgb_to_hex(0, 0, 0);
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
