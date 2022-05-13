#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

fn abs(val: f32) -> f32 {
    if val < 0.0 {
        return val * -1.0;
    } else {
        return val;
    }
}

pub fn rgb(r: u8, g: u8, b: u8) -> Color {
    return Color {
        r: r,
        g: g,
        b: b,
    };
}

pub fn hsv(hue: f32) -> Color {
    let v = 100.0;
    let s = 100.0;
    let theta = match hue > 360.0 {
        true => 0.0,
        false => hue,
    };
    
    let c = v * s;
    let x = (c * (1.0 - abs(theta / 60.0) % 2.0 - 1.0));
    let m = v - c;

    let mut rgb_prime = (0.0, 0.0, 0.0);

    if hue > 0.0 && hue < 60.0 {
        rgb_prime = (c, x, 0.0);
    } else if hue >= 60.0 && hue < 120.0 {
        rgb_prime = (x, c, 0.0);
    } else if hue >= 120.0 && hue < 180.0 {
        rgb_prime = (0.0,  c,  x);
    } else if hue >= 180.0 && hue < 240.0 {
        rgb_prime = (0.0, x, c);
    } else if hue >= 240.0 && hue < 300.0 {
        rgb_prime = (x, 0.0, c);
    } else {
        rgb_prime = (c, 0.0, x);
    }

    return rgb(
        ((rgb_prime.0 + m) * 255.0) as u8,
        ((rgb_prime.1 + m) * 255.0) as u8,
        ((rgb_prime.2 + m) * 255.0) as u8,
    );
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

    pub fn adjust(&self) -> Self {
        let mut r: i32 = self.r as i32;
        let mut g: i32 = self.g as i32;
        let mut b: i32 = self.b as i32;

        if self.r > 180 || self.g > 180 || self.b > 180 {
            // Scale down
            let scale = teensycore::math::max(teensycore::math::max(g, b) as i32, r as i32) - 180;
            r -= scale;
            g -= scale;
            b -= scale;
        }

        if r < 0 {
            b += -r;
            r = 20;
        } 

        if g < 0 {
            r += -g;
            g = 20;
        }

        if b < 0 {
            g += -b;
            b = 20;
        }

        b = teensycore::math::min(b, 180);
        g = teensycore::math::min(g, 180);
        r = teensycore::math::min(r, 180);

        return rgb(r as u8, g as u8, b as u8);
    }
}