use teensycore::mem::*;
use teensycore::clock::uNano;
use crate::pixel_engine::color::*;
use crate::pixel_engine::math::*;

#[derive(Copy, Clone)]
struct ShaderStep {
    time: uNano,
    color: Color,
    next: Option<*mut ShaderStep>,
}

#[derive(Copy, Clone)]
pub struct Shader {
    pub name: &'static [u8],
    sealed: bool,
    color: Color,
    root: Option<*mut ShaderStep>,
    pub total_segments: usize,
    pub total_time: uNano,
    pub wifi_only: bool,
    pub disabled: bool,
}

impl Shader {

    pub fn new(name: &'static [u8]) -> Self {
        return Shader{
            name: name,
            sealed: false,
            disabled: false,
            color: rgb(0, 0, 0),
            root: None,
            total_segments: 0,
            total_time: 0,
            wifi_only: false,
        }.clone();
    }

    fn add_node(&mut self, step: ShaderStep) {
        let ptr = alloc();
        unsafe {
            (*ptr) = step;
        }

        // Iterate through head looking for the tail
        if self.root.is_none() {
            self.root = Some(ptr);
        } else {
            let mut tail_ptr = self.root.unwrap();
            while unsafe { tail_ptr.as_mut().unwrap().next.is_some() } {
                tail_ptr = unsafe { (*tail_ptr).next.unwrap() };
            }

            unsafe { (*tail_ptr).next = Some(ptr) };
        }

        self.total_segments += 1;
        self.total_time += step.time;
    }

    pub fn set_segment_count(&mut self, segment_count: usize) -> &mut Self {
        self.total_segments = segment_count;
        return self;
    }

    pub fn as_wifi_only(&mut self) -> &mut Self {
        self.wifi_only = true;
        return self;
    }

    pub fn as_disabled(&mut self) -> &mut Self {
        self.disabled = true;
        return self;
    }

    pub fn with_color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        return self;
    }

    pub fn transition_to(&mut self, color: Color, time: uNano) -> &mut Self {
        if self.sealed {
            return self;
        }

        self.add_node(ShaderStep {
            time: time,
            color: color,
            next: None,
        });

        return self;
    }

    pub fn merge(&mut self, other: &Shader) {
        let mut ptr = other.root;
        while ptr.is_some() {
            let node = unsafe { *ptr.unwrap() };
            self.add_node(node);
            ptr = node.next;
        }
    }

    pub fn build(&mut self) -> Self {
        self.sealed = true;
        return self.clone();
    }

    pub fn get_color(&mut self, time: uNano) -> Color {
        let normalized_time = time % self.total_time;
        // Now the interpolation begins
        // Find the node we care about
        
        // Find the target node
        if self.root.is_none() {
            return self.color;
        } else {
            let mut color = self.color;
            let mut ptr = self.root.unwrap();
            let mut elapsed = 0;

            while elapsed + unsafe { (*ptr).time } < normalized_time {
                color = unsafe { (*ptr).color };
                elapsed += unsafe { (*ptr).time };
                ptr = unsafe { (*ptr).next.unwrap() };
            }

            // Compute the new rgb
            let next_color = unsafe { (*ptr).color };
            let duration = unsafe { (*ptr).time };

            let r = interpolate(color.r as u32, next_color.r as u32, normalized_time - elapsed, duration);
            let g = interpolate(color.g as u32, next_color.g as u32, normalized_time - elapsed, duration);
            let b = interpolate(color.b as u32, next_color.b as u32, normalized_time - elapsed, duration);

            let (h, s, v) = rgb_to_hsv(r as u8, g as u8, b as u8);
            return hsv(h, teensycore::math::max(s, 0.6), v);

            // return rgb(r as u8, g as u8, b as u8);
            // return hsv(next_h as f32, 1.0, 1.0);
        }
    }
}

#[cfg(test)]
pub mod test_shaders {
    
    use super::*;
    use teensycore::*;
    use teensycore::system::str::*;

    #[test]
    fn test_shader() {
        // let mut shader = Shader::new(b"Sample")
        //     .with_color(rgb(255, 0, 0))
        //     .transition_to(rgb(0,255,0), 500)
        //     .transition_to(rgb(255,0,0), 500)
        //     .build();

        // let color = shader.get_color(250);
        // assert_eq!(color.r, 128);
        // assert_eq!(color.g, 127);
        // assert_eq!(color.b, 0);

        // let color2 = shader.get_color(503);
        // assert_eq!(color2.r, 1);
    }
}