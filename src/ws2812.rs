pub mod shader;

use crate::shader::*;
use crate::{drivers::ws2812::*, proc_handle, models::SystemCommand};
use teensycore::{clock::*, debug::debug_str, system::closure::Closure};
use teensycore::math::*;

const LEDS: usize = 4;

static mut BASIC_SHADER: BasicShader = BasicShader::new();
static mut XMAS_SHADER: XmasShader = XmasShader::new();
static mut CONSTRAINED_RAINBOW_SHADER: ConstrainedRainbowShader = ConstrainedRainbowShader::new();
static mut AUDIO_EQUALIZER_SHADER: AudioEqualizerShader = AudioEqualizerShader::new();

fn get_shader(shader: ActiveShader) -> &'static mut dyn Shader::<LEDS> {
    return match shader {
        ActiveShader::Basic => unsafe { &mut BASIC_SHADER }, 
        ActiveShader::Xmas => unsafe { &mut XMAS_SHADER },
        ActiveShader::Constrained => unsafe { &mut CONSTRAINED_RAINBOW_SHADER },
        ActiveShader::AudioEqualizer => unsafe { &mut AUDIO_EQUALIZER_SHADER },
    };
}

pub struct Interpolator {
    start_colors: [u32; LEDS],
    end_colors: [u32; LEDS],
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
            return delta;
        }
        
    }
}

#[derive(Copy, Clone)]
pub enum ActiveShader {
    Basic = 0x0,
    Xmas = 0x1,
    Constrained = 0x2,
    AudioEqualizer = 0x3,
}

impl ActiveShader {
    pub fn list() -> [ActiveShader; 4] {
        return [
            ActiveShader::Basic,
            ActiveShader::Xmas,
            ActiveShader::Constrained,
            ActiveShader::AudioEqualizer,
        ];
    }
}


pub struct WS2812Task { 
    target: u64,
    driver: WS2812Driver<LEDS>,
    shader: ActiveShader,
    contexts: [ShaderContext; LEDS],
    interpolator: Interpolator,
    speed: u64,
}

static mut TASK_INSTANCE: WS2812Task = WS2812Task {
    target: 0,
    driver: WS2812Driver::<LEDS>::new(
        18, // pin
    ),
    shader: ActiveShader::Basic,
    speed: teensycore::MS_TO_NANO * 18,
    contexts: [ShaderContext::new(0, LEDS); LEDS],
    interpolator: Interpolator  {
        begin_time: 0,
        start_colors: [0; LEDS],
        end_colors: [0; LEDS],
        duration: 0,
    },
};

impl WS2812Task {

    pub fn set_shader(&mut self, shader: ActiveShader) {
        let active_shader = get_shader(shader);
        for i in 0 .. LEDS {
            self.contexts[i] = active_shader.init(self.contexts[i]);
        }
        self.shader = shader;
    }

    pub fn interpolate_to(&mut self, next_shader: ActiveShader, registers: [i32; 10]) {
        // Store current values
        for i in 0 .. LEDS {
            // We are interpolating /wheel/ not colors.
            // so access register[0] which is always allocated
            // for wheel.
            self.interpolator.start_colors[i] = self.contexts[i].registers[0] as u32;
        }

        // Initialize next colors
        let active_shader = get_shader(next_shader);
        for i in 0 .. LEDS {
            // Update registers based on provided data
            self.contexts[i].registers = registers;
            self.contexts[i] = active_shader.init(self.contexts[i]);
            self.interpolator.end_colors[i] = self.contexts[i].registers[0] as u32;
        }

        // Update the shader
        self.interpolator.begin_time = nanos();
        self.shader = next_shader;
    }

    pub fn get_instance<'a>() -> &'a mut WS2812Task {
        return unsafe { &mut TASK_INSTANCE };
    }

    pub fn init(&mut self) {
        for idx in 0 .. LEDS {
            self.contexts[idx].node_id = idx;
            self.contexts[idx].total_nodes = LEDS;
        }

        self.driver.init();
        self.set_shader(self.shader);

        proc_handle(&|cmd: &SystemCommand| {
            debug_str(b"message received!!");
            debug_str(&cmd.command);

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
                        3 => {
                            instance.interpolate_to(ActiveShader::AudioEqualizer, cmd.args);
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

        if time > self.target {

            // Check if we're interpolating
            let interpolating = (self.interpolator.begin_time + self.interpolator.duration) > time;
            if interpolating {
                // Process the interpolation
                for i in 0 .. LEDS {
                    let wheel_index = self.interpolator.interpolate(i, time);
                    self.driver.set_color(i, wheel(wheel_index as u8));
                }
            } else {
                // Process the shader as normal
                let active_shader = get_shader(self.shader);
                for i in 0 .. LEDS {
                    self.contexts[i] = active_shader.update(self.contexts[i]);
                    self.driver.set_color(i, self.contexts[i].color);
                }
            }


            self.driver.flush();
            self.target = nanos() + self.speed;
        }
    }
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