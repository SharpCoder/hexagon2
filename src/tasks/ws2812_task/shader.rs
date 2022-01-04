use crate::drivers::ws2812::*;

#[derive(Copy, Clone)]
pub struct ShaderContext {
    pub node_id: usize,
    pub total_nodes: usize,
    pub current_time: u64,
    pub temperature: i32,
    pub registers: [i32; 10],
    pub color: u32,
}

impl ShaderContext {
    pub fn new(id: usize, total_nodes: usize) -> Self {
        return ShaderContext {
            node_id: id,
            total_nodes: total_nodes,
            current_time: crate::clock::nanos(),
            temperature: 0,
            registers: [0; 10],
            color: 0xFF0000,
        }
    }
}

pub trait Shader<const SIZE: usize> {
    fn init(&mut self, context: ShaderContext) -> ShaderContext;
    fn update(&mut self, context: ShaderContext) -> ShaderContext;
}

/**
Basic rainbow shader
*/
pub struct BasicShader {
    count: u8,
}

impl <const SIZE: usize> Shader<SIZE> for BasicShader {
    fn init(&mut self, context: ShaderContext) -> ShaderContext {
        return context;
    }

    fn update(&mut self, context: ShaderContext) -> ShaderContext {
        let mut next_context: ShaderContext = context;
        next_context.color = wheel(self.count + (context.node_id / context.total_nodes) as u8 );
        self.count += 1;
        return next_context;
    }
}

impl BasicShader {
    pub const fn new() -> BasicShader {
        return BasicShader {
            count: 0,
        };
    }
}


/**
Xmas themed shader
*/
pub struct XmasShader {
    count: u8,
}
impl XmasShader {
    pub const fn new() -> XmasShader {
        return XmasShader {
            count: 0,
        };
    }
}
impl <const SIZE: usize> Shader<SIZE> for XmasShader {
    fn init(&mut self, context: ShaderContext) -> ShaderContext {
        let mut next_context = context;
        // Randomize the starting position for each node
        next_context.registers[0] = (crate::math::rand() % 170) as i32;
        return next_context;
    }
    
    fn update(&mut self, context: ShaderContext) -> ShaderContext {
        let mut next_context: ShaderContext = context;
        let pos = context.registers[0] as u8;// + (context.node_id / SIZE) as u8;

        // R -> G -> R wheel
        if pos < 85 {
            next_context.color = rgb_to_hex(255 - pos * 3, pos * 3, 0);
        } else if pos < 170 {
            next_context.color = rgb_to_hex((pos - 85) * 3, 255 - (pos - 85) * 3, 0);
        } else {
            self.count = 0;
            next_context.color = rgb_to_hex(252, 3, 0);
        }

        next_context.registers[0] += 1;
        return next_context;
    }
}
