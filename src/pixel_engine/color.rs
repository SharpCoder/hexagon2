use teensycore::math::{max, min};

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

pub fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let rgb_prime = (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
    let c_max = max(max(rgb_prime.0, rgb_prime.1), rgb_prime.2);
    let c_min = min(min(rgb_prime.0, rgb_prime.1), rgb_prime.2);
    let delta = c_max - c_min;

    let v = c_max;
    let mut h;
    let s = match c_max == 0.0 || delta == 0.0{
        true => 0.0,
        false => delta / c_max,
    };

    // assert_eq!(c_min, 12.0);

    if delta == 0.0 {
        h = 0.0;
    } else if c_max == rgb_prime.0 {
        // R is dominant
        h = 60.0 * (((rgb_prime.1 - rgb_prime.2) / delta) % 6.0);
    } else if c_max == rgb_prime.1 {
        // G is dominant
        h = 60.0 * (((rgb_prime.2 - rgb_prime.0) / delta) + 2.0);
    } else {
        // B is dominant
        h = 60.0 * (((rgb_prime.0 - rgb_prime.1) / delta) + 4.0);
    }

    
    if h < 0.0 {
        h = 360.0 + h;
    } 
    

    if h  > 360.0 {
        h = h - 360.0;
    }

    return (h, s, v);
}

pub fn hsv(h: f32, s: f32, v: f32) -> Color {
    let theta = h % 360.0;    
    let c = v * s;
    let x = c * (1.0 - abs(((theta / 60.0) % 2.0) - 1.0));
    let m = v - c;
    
    let rgb_prime;

    if theta >= 0.0 && theta < 60.0 {
        rgb_prime = (c, x, 0.0);
    } else if theta >= 60.0 && theta < 120.0 {
        rgb_prime = (x, c, 0.0);
    } else if theta >= 120.0 && theta < 180.0 {
        rgb_prime = (0.0,  c,  x);
    } else if theta >= 180.0 && theta < 240.0 {
        rgb_prime = (0.0, x, c);
    } else if theta >= 240.0 && theta < 300.0 {
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
}

#[cfg(test)]
pub mod test_colors {
    use crate::pixel_engine::math::interpolate;
    use std::println;
    use super::*;

    #[test]
    fn test_hsv() {
        // assert_eq!(hsv(39.0, 1.0, 1.0).g, 165);

        // let (h, s, v) = rgb_to_hsv(0,0,128);
        // assert_eq!(h, 240.0);
        // assert_eq!(s, 1.0);
        // assert_eq!(v, 0.5019608);
        // assert_eq!(hsv(h, s, v).b, 128);
        // assert_eq!(hsv(0.0).r, 255);
    }

    #[test]
    fn test_interpolated_hsv() {


        // // Interpolate
        // for time in 0 .. 100 {
        //     let start = (255, 0, 0);
        //     let end = (50, 0, 255);
        //     let r = interpolate(start.0, end.0, time, 100);
        //     let g = interpolate(start.1, end.1, time, 100);
        //     let b = interpolate(start.2, end.2, time, 100);
        //     let (h, s, v) = rgb_to_hsv(r as u8, g as u8, b as u8);

        //     println!("rgb({}, {}, {})  hsv({}, {}, {})", r, g, b, h, s, v);
        // }

        // for time in 0 .. 100 {
        //     let start = (50, 0, 255);
        //     let end = (255, 0, 0);
        //     let r = interpolate(start.0, end.0, time, 100);
        //     let g = interpolate(start.1, end.1, time, 100);
        //     let b = interpolate(start.2, end.2, time, 100);
        //     let (h, s, v) = rgb_to_hsv(r as u8, g as u8, b as u8);

        //     println!("rgb({}, {}, {})  hsv({}, {}, {})", r, g, b, h, s, v);
        // }

        std::println!("hi");
        let (h, s, v) = rgb_to_hsv(148, 0, 133);
        // assert_eq!(h, 306.0);
    }
}