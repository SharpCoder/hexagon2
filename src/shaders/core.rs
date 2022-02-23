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
    fn randomize(&self, context: ShaderContext) -> ShaderContext;
    fn init(&mut self, context: ShaderContext) -> ShaderContext;
    fn update(&mut self, context: ShaderContext) -> ShaderContext;
}


pub fn hex_to_rgb(rgb: u32) -> (u32, u32, u32) {
    let r = ((rgb & 0xFF0000) << 16);
    let g = ((rgb & 0x00FF00) << 8);
    let b = (rgb & 0x0000FF);

    return (r, g, b);
}

pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> u32 {
    return ((g as u32) << 16) |
        ((r as u32) << 8) |
        (b as u32); 
}

pub fn wheel(wheel_pos: u8) -> u32 {
    let mut pos = wheel_pos;
    if pos < 85 {
        return rgb_to_hex(255 - pos * 3, 0, pos * 3);
    } else if pos < 170 {
        pos -= 85;
        return rgb_to_hex(0, pos * 3, 255 - pos * 3);
    } else {
        pos -= 170;
        return rgb_to_hex(pos * 3, 255 - pos * 3, 0);
    }
}
