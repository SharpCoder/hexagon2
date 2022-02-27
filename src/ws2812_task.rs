use crate::shaders::{
    core::*,
    basic::*,
    constrained::*,
    xmas::*,
};

use crate::{drivers::ws2812::*, proc_handle, models::SystemCommand};
use teensycore::debug::debug_u64;
use teensycore::phys::irq::{disable_interrupts, enable_interrupts};
use teensycore::{clock::*, debug::debug_str, system::closure::Closure};
use teensycore::{math::{self, *}, S_TO_NANO, MS_TO_NANO};

const LEDS: usize = 9;
const LED_PER_UNIT: usize = 3;
const UNITS: usize = LEDS / LED_PER_UNIT;

static mut BASIC_SHADER: BasicShader = BasicShader::new();
static mut XMAS_SHADER: XmasShader = XmasShader::new();
static mut CONSTRAINED_RAINBOW_SHADER: ConstrainedRainbowShader = ConstrainedRainbowShader::new();

fn get_shader(shader: ActiveShader) -> &'static mut dyn Shader::<UNITS> {
    return match shader {
        ActiveShader::Basic => unsafe { &mut BASIC_SHADER }, 
        ActiveShader::Xmas => unsafe { &mut XMAS_SHADER },
        ActiveShader::Constrained => unsafe { &mut CONSTRAINED_RAINBOW_SHADER },
    };
}

pub struct Interpolator {
    start_colors: [u32; UNITS],
    end_colors: [u32; UNITS],
    duration: u64,
    begin_time: u64,
}

impl Interpolator {
    pub fn interpolate(&self, index: usize, current_time: u64) -> u32 {
        // Calculate step
        let x0 = 0f32;
        let y0 = min(self.start_colors[index], self.end_colors[index]) as f32;
        let x1 = self.duration as f32;
        let y1 = max(self.end_colors[index], self.start_colors[index]) as f32;
        let x = min(current_time - self.begin_time, self.duration) as f32;
        let delta = (x  * ((y1 - y0)/(x1 - x0))) as u32;

        // Check if it's reversed. This is necessary because of
        // integer division and stuff.
        if self.start_colors[index] > self.end_colors[index] {
            return self.start_colors[index] - delta;
        } else {
            return self.start_colors[index] + delta;
        }
        
    }
}

#[derive(Copy, Clone)]
pub enum ActiveShader {
    Basic = 0x0,
    Xmas = 0x1,
    Constrained = 0x2,
}

impl ActiveShader {
    pub fn list() -> [ActiveShader; 3] {
        return [
            ActiveShader::Basic,
            ActiveShader::Xmas,
            ActiveShader::Constrained,
        ];
    }
}


pub struct WS2812Task { 
    target: u64,
    transition_target: u64,
    driver: WS2812Driver<LEDS>,
    shader: ActiveShader,
    contexts: [ShaderContext; UNITS],
    interpolator: Interpolator,
    speed: u64,
}

static mut TASK_INSTANCE: WS2812Task = WS2812Task {
    target: 0,
    transition_target: 0,
    driver: WS2812Driver::<LEDS>::new(
        18, // pin
    ),
    shader: ActiveShader::Basic,
    speed: teensycore::MS_TO_NANO * 8,
    contexts: [ShaderContext::new(0, UNITS); UNITS],
    interpolator: Interpolator  {
        begin_time: 0,
        start_colors: [0; UNITS],
        end_colors: [0; UNITS],
        duration: 0,
    },
};

impl WS2812Task {

    pub fn iterate(&mut self) {
        self.driver.iterate();
    }

    pub fn set_shader(&mut self, shader: ActiveShader) {
        let active_shader = get_shader(shader);
        for i in 0 .. UNITS {
            self.contexts[i] = active_shader.init(self.contexts[i]);
        }
        self.shader = shader;
    }

    pub fn interpolate_to(&mut self, next_shader: ActiveShader, registers: [i32; 10]) {
        // Store current values
        for i in 0 .. UNITS {
            // We are interpolating /wheel/ not colors
            self.interpolator.start_colors[i] = find_wheel_pos(self.contexts[i].color) as u32;
        }

        // Initialize next colors
        let active_shader = get_shader(next_shader);
        for i in 0 .. UNITS {
            // Update registers based on provided data
            self.contexts[i] = active_shader.init(self.contexts[i]);
            self.contexts[i] = active_shader.randomize(self.contexts[i]);
            
            // Copy any registers that aren't i32::MAX which
            // is the default value specified in parser..
            // ALSO!!! The first register is not what you think it is.
            // Reg[0] is typically important to not wipe out.
            for r in 1 .. 10 {
                if registers[r] != i32::MAX {
                    self.contexts[i].registers[r] = registers[r];
                }
            }

            self.contexts[i] = active_shader.update(self.contexts[i]);
            self.interpolator.end_colors[i] = find_wheel_pos(self.contexts[i].color) as u32;
        }

        // Update the shader
        self.interpolator.begin_time = nanos();
        self.interpolator.duration = MS_TO_NANO * 850;
        self.shader = next_shader;
    }

    pub fn get_instance<'a>() -> &'a mut WS2812Task {
        return unsafe { &mut TASK_INSTANCE };
    }

    pub fn init(&mut self) {
        for idx in 0 .. UNITS {
            self.contexts[idx].node_id = idx;
            self.contexts[idx].total_nodes = UNITS;
        }

        self.driver.init();
        self.set_shader(self.shader);

        proc_handle(&|cmd: &SystemCommand| {
            let instance = WS2812Task::get_instance();
            match &cmd.command {
                b"SETS" => {
                    match cmd.args[0] {
                        0 => {
                            instance.interpolate_to(ActiveShader::Basic, cmd.args);
                        },
                        1 => {
                            instance.interpolate_to(ActiveShader::Constrained, cmd.args);
                        },
                        2 => {
                            instance.interpolate_to(ActiveShader::Xmas, cmd.args);
                        },
                        _ => {
                            
                        }
                    }
                },
                b"TIME" => {
                    let ms = min(max(cmd.args[0], 1000), 1);
                    instance.speed = (ms as u64) * teensycore::MS_TO_NANO;
                },
                _ => {

                }
            }
        });

    }

    pub fn system_loop(&mut self) {
        let time = nanos();

        if time > self.transition_target {
            // Transition to
            self.transition_target = time + S_TO_NANO * 10;
            let rnd = rand() % 3;   
            let instance = WS2812Task::get_instance();
            match rnd {
                0 => {
                    instance.interpolate_to(ActiveShader::Basic, [i32::MAX; 10]);
                },
                1 => {
                    instance.interpolate_to(ActiveShader::Basic, [i32::MAX; 10]);
                },
                _ => {
                    instance.interpolate_to(ActiveShader::Basic, [i32::MAX; 10]);
                }
            }
        }

        if time > self.target {
            // Check if we're interpolating
            let interpolating = (self.interpolator.begin_time + self.interpolator.duration) > time;
            if interpolating {
                // Process the interpolation
                for i in 0 .. UNITS {
                    let wheel_index = self.interpolator.interpolate(i, time);
                    for r in 0 .. LED_PER_UNIT {
                        self.driver.set_color(i * LED_PER_UNIT + r, wheel(wheel_index as u8));
                    }
                }
            } else {
                // Process the shader as normal
                let active_shader = get_shader(self.shader);
                for i in 0 .. UNITS {
                    self.contexts[i] = active_shader.update(self.contexts[i]);
                    for r in 0 .. LED_PER_UNIT {
                        self.driver.set_color(i * LED_PER_UNIT + r, self.contexts[i].color);
                    }
                }
            }


            // Turn off the first LED
            self.driver.set_color(0, 0x00);

            self.driver.flush();
            self.target = nanos() + self.speed;
        }
    }
}

/// Given rgb, find the closest wheel setting to that
fn find_wheel_pos(rgb: u32) -> u8 {
    let (r, g, b) = hex_to_rgb(rgb);
    let mut final_pos = 0;
    let mut final_dist = u64::MAX;

    for i in 0 .. 255 {
        let (r2, g2, b2) = hex_to_rgb(wheel(i));

        let dist = unsafe {
            math::pow(max(r2, r) as u64 - min(r2, r) as u64, 2) +
            math::pow(max(g2, g) as u64 - min(g2, g) as u64, 2) +
            math::pow(max(b2, b) as u64 - min(b2, b) as u64, 2)
        };

        if dist < final_dist {
            final_pos = i;
            final_dist = dist;
        }
    }

    return final_pos as u8;
}

#[cfg(test)]
pub mod shaders {
    
    use super::*;

    #[test]
    fn test_interpolation() {
        let interpolator = Interpolator {
            start_colors: [0; LEDS],
            end_colors: [100; LEDS],
            duration: 1000,
            begin_time: 0,
        };

        
        assert_eq!(interpolator.interpolate(0, 250), 25);
        assert_eq!(interpolator.interpolate(0, 370), 37);
        assert_eq!(interpolator.interpolate(0, 480), 48);
        assert_eq!(interpolator.interpolate(0, 1000), 100);
        assert_eq!(interpolator.interpolate(0, 1020), 100);
    }

    #[test]
    fn test_reverse_interpolation() {
        let interpolator = Interpolator {
            start_colors: [100; LEDS],
            end_colors: [0; LEDS],
            duration: 1000,
            begin_time: 0,
        };

        
        assert_eq!(interpolator.interpolate(0, 250), 75);
        assert_eq!(interpolator.interpolate(0, 370), 63);
        assert_eq!(interpolator.interpolate(0, 480), 52);
        assert_eq!(interpolator.interpolate(0, 1000), 0);
        assert_eq!(interpolator.interpolate(0, 1020), 0);

    }
}