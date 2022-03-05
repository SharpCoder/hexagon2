#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub fn rgb(r: u8, g: u8, b: u8) -> Color {
    return Color {
        r: r,
        g: g,
        b: b,
    };
}

impl Color {
    pub fn as_hex(&self) -> u32 {
        return ((self.r as u32) << 16) |
            ((self.g as u32) << 8) |
            (self.b as u32); 
    }

    pub fn blank() -> Self {
        return Color {
            r: 255,
            g: 255,
            b: 255,
        }
    }
}