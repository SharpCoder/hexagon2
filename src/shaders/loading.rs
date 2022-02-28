use teensycore::math::rand;

use crate::shaders::core::*;

pub struct LoadingShader {}
impl LoadingShader {
    pub const fn new() -> LoadingShader {
        return LoadingShader {};
    }
}
impl <const SIZE: usize> Shader<SIZE> for LoadingShader {
    fn name(&self) -> &[u8] { return b"Loading"; }

    fn init(&mut self, context: ShaderContext) -> ShaderContext {
        let mut next_context = context;
        

        // This code basically takes a pattern like
        // 0 1 2 3 4 5 6 7 8 9
        // And makes it
        // 5 4 3 2 1 0 1 2 3 4
        // Which would have the effect of a ripple starting from the center
        // and going outwards.
        let origin = context.total_nodes / 2;
        let adj_pos = (context.node_id + origin) % context.total_nodes;
        let pos;

        if adj_pos > origin {
            pos = context.total_nodes - adj_pos;
        } else {
            pos = adj_pos;
        }

        next_context.registers[0] = (255i32 / (context.total_nodes / 2) as i32) * pos as i32;

        // Determine mode
        // 0 = Rainbow
        // 1 = Color
        next_context.registers[1] = (rand() % 2) as i32;

        return next_context;
    }
    
    fn update(&mut self, context: ShaderContext) -> ShaderContext {
        let mut next_context: ShaderContext = context;
        let pos = context.registers[0] as u32;

        if context.registers[1] == 1 {
            if pos < 85 {
                // Green -> Blue
                next_context.color = rgb_to_hex(
                    (pos * 3) as u8,
                    255 - (pos * 3) as u8, 
                    0
                );
            } else if pos < 170 {
                // Blue -> Green
                next_context.color = rgb_to_hex(
                    255 - (pos * 3) as u8,
                    (pos * 3) as u8, 
                    0
                );
            } else {
                next_context.registers[0] = 0;
                next_context.color = rgb_to_hex(0, 255, 0);
            }
        } else {
            if pos >= 254 {
                next_context.registers[0] = 0;
                next_context.color = wheel(0);
            } else {
                next_context.color = wheel((pos * 2) as u8);
            }
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
