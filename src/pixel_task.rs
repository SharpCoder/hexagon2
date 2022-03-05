
use teensycore::MS_TO_NANO;
use teensycore::clock::*;
use teensycore::math::rand;
use teensycore::system::vector::Vector;
use crate::pixel_engine::color::*;
use crate::pixel_engine::math::interpolate;
use crate::pixel_engine::shader::*;
use crate::pixel_engine::effect::*;
use crate::pixel_engine::context::*;
use crate::drivers::ws2812::*;
use crate::shaders::initialize_shaders;

const UNITS: usize = 7;
const LEDS_PER_UNIT: usize = 3;
const LEDS: usize = UNITS * LEDS_PER_UNIT;
const TRANSITION_TIME: u64 = 3750; // ms

pub struct PixelTask {
    shader: Option<Shader>,
    next_shader: Option<Shader>,
    shaders: Vector<Shader>,
    contexts: [Context; UNITS],
    effect: Effect,
    driver: WS2812Driver<LEDS>,
    target: u64,
    ready: bool,
    time_offset: u64,
    transition_start: u64,
    transitioning: bool,
}

impl PixelTask {
    pub fn new() -> Self {

        return PixelTask {
            target: 0,
            time_offset: 0,
            transition_start: 0,
            ready: false,
            transitioning: false,
            shader: None,
            next_shader: None,
            shaders: initialize_shaders(),
            driver: WS2812Driver::<LEDS>::new(
                18, // pin
            ),
            contexts: [Context::empty(); UNITS],
            effect: Effect::new(b"Randomized")
                .with_initializer(|ctx| {
                    let mut next_ctx = ctx.clone();
                    next_ctx.offset = (rand() % 3) as u64 * 1000;
                    return next_ctx;
                })
                .transition_to(100, 3000)
                .build(),
        };   
    }

    fn find_shader(&self, name: &'static [u8]) -> Option<Shader> {
        for shader in self.shaders.into_iter() {
            if shader.name == name {
                return Some(shader);
            }
        }

        return None;

    }

    // Evaluate which shader to select based on
    // world information.
    fn get_next_shader(&self) -> Option<Shader> {
        return self.find_shader(b"Mars");
    }

    pub fn init(&mut self) {
        self.driver.init();

        // Initialize the contexts
        for node_id in 0 .. UNITS {
            self.contexts[node_id].node_id = node_id;
            self.contexts[node_id].total_nodes = UNITS;
            self.contexts[node_id].initialized = false;
        }

        // Select a shader
        self.shader = self.find_shader(b"Medbay");
    }

    pub fn system_loop(&mut self) {
        let time = nanos();
        if time > self.target {
            if self.transitioning {
                if time > (self.transition_start + TRANSITION_TIME * teensycore::MS_TO_NANO) {
                    self.transitioning = false;
                    self.shader = self.next_shader;
                    self.time_offset = self.transition_start;
                    teensycore::debug::blink_accumulate();
                }
            }
            
            let elapsed_ms = (time - self.time_offset) / teensycore::MS_TO_NANO;
            match self.shader.as_mut() {
                None => {},
                Some(shader) => {
                    for node_id in 0 .. UNITS {
                        let mut ctx = self.contexts[node_id];
                        let (time_t, next_context) = self.effect.process(&mut ctx, elapsed_ms);
                        let time_t = (( time_t as f64 / 100.0) * shader.total_time as f64) as u64;

                        let color;
                        
                        // If we are transitioning, let's interpolate the colors
                        // from where they were when transition started, to where
                        // they should be as transition continues
                        //
                        // Over the time period of TRANSITION_TIME.
                        if self.transitioning {
                            let next_shader = self.next_shader.as_mut().unwrap();
                            let time_t_transition = (time - self.transition_start) / teensycore::MS_TO_NANO;
                            let time_ka = (self.effect.process(&mut ctx, time_t_transition).0 as f64 / 100.0);

                            // Compute the colors based on the transition timeline
                            let original_color = shader.get_color((time_ka * shader.total_time as f64) as u64);
                            let next_color = next_shader.get_color((time_ka * next_shader.total_time as f64) as u64);

                            // Interpolate the rgb values to produce a smooth
                            // looking transition between the two
                            // sequences.
                            color = rgb(
                                interpolate(original_color.r as u32, next_color.r as u32, time_t_transition, TRANSITION_TIME) as u8,
                                interpolate(original_color.g as u32, next_color.g as u32, time_t_transition, TRANSITION_TIME) as u8,
                                interpolate(original_color.b as u32, next_color.b as u32, time_t_transition, TRANSITION_TIME) as u8,
                            ).as_hex();
                            

                        } else {
                            color = shader.get_color(time_t).as_hex();
                        }
                        
                        self.contexts[node_id] = next_context;
                        for pixel_id in 0 .. LEDS_PER_UNIT {
                            self.driver.set_color(node_id * LEDS_PER_UNIT + pixel_id, color);
                        }
                    }
            
                    self.driver.flush();
                    self.target = nanos() + 7 * teensycore::MS_TO_NANO;
                }
            }
        }
    }

    pub fn ready(&mut self) {
        if !self.ready {
            self.ready = true;
            // Create a custom shader to transition from current color
            // to future color1
            let next_shader = match self.get_next_shader() {
                None => self.find_shader(b"Mars").unwrap(), // TODO: Special error shader
                Some(shader)  => shader,
            };

            self.next_shader = Some(next_shader);
            self.transitioning = true;
            self.transition_start = nanos();
        }
    }

}