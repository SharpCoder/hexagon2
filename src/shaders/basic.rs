use crate::shaders::core::*;

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

    fn randomize(&self, context: ShaderContext) -> ShaderContext {
        return context;
    }
}

impl BasicShader {
    pub const fn new() -> BasicShader {
        return BasicShader {};
    }
}