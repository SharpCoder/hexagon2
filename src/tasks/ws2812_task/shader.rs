use crate::drivers::ws2812::*;

#[derive(Copy, Clone)]
pub struct ShaderContext {
    pub node_id: usize,
    pub total_nodes: usize,
    pub current_time: u64,
    pub temperature: i32,
}

pub trait Shader<const SIZE: usize> {
    fn update(&mut self, driver: &mut WS2812Driver::<SIZE>, context: ShaderContext);
}

pub struct BasicShader {
    count: u8,
}
impl <const SIZE: usize> Shader<SIZE> for BasicShader {
    fn update(&mut self, driver: &mut WS2812Driver::<SIZE>, context: ShaderContext) {
        let color = wheel(self.count + (context.node_id / context.total_nodes) as u8 );
        (*driver).set_color(context.node_id, color);
        self.count += 1;
    }
}
impl BasicShader {
    pub const fn new() -> BasicShader {
        return BasicShader {
            count: 0,
        };
    }
}

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
    fn update(&mut self, driver: &mut WS2812Driver::<SIZE>, context: ShaderContext) {
        let pos = self.count + (context.node_id / SIZE) as u8;
        let color: u32;

        if pos < 85 {
            color = rgb_to_hex(255 - pos * 3, pos * 3, 0);
        } else if pos < 170 {
            color = rgb_to_hex((pos - 85) * 3, 255 - (pos - 85) * 3, 0);
        } else {
            self.count = 0;
            color = rgb_to_hex(255, 0, 0);
        }

        (*driver).set_color(context.node_id, color);
        self.count += 1;
    }
}
