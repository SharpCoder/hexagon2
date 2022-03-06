
use teensycore::MS_TO_NANO;
use teensycore::clock::*;
use teensycore::math::rand;
use teensycore::system::vector::Array;
use teensycore::system::vector::Vector;
use crate::shaders::*;
use crate::effects::*;
use crate::pixel_engine::color::*;
use crate::pixel_engine::math::interpolate;
use crate::pixel_engine::shader::*;
use crate::pixel_engine::effect::*;
use crate::pixel_engine::context::*;
use crate::drivers::ws2812::*;

const UNITS: usize = 9;
const LEDS_PER_UNIT: usize = 3;
const LEDS: usize = UNITS * LEDS_PER_UNIT;
const TRANSITION_TIME: u64 = 1000; // ms

enum PixelState {
    Loading,
    Transitioning,
    MainSequence,
}

pub struct PixelTask {
    state: PixelState,
    shader: Option<Shader>,
    next_shader: Option<Shader>,
    shaders: Vector<Shader>,
    contexts: [Context; UNITS],
    effect: Option<Effect>,
    effects: Vector<Effect>,
    driver: WS2812Driver<LEDS>,
    target: u64,
    ready: bool,
    color_buffer: [Color; UNITS],
    transition_start: u64,
    transition_offset: u64,
}

impl PixelTask {
    pub fn new() -> Self {

        return PixelTask {
            state: PixelState::Loading,
            target: 0,
            transition_start: 0,
            transition_offset: 0,
            ready: false,
            shader: None,
            effect: None,
            next_shader: None,
            shaders: initialize_shaders(),
            effects: initialize_effects(),
            driver: WS2812Driver::<LEDS>::new(
                18, // pin
            ),
            color_buffer: [Color::blank(); UNITS],
            contexts: [Context::empty(); UNITS],
        };   
    }

    fn find_effect(&self, name: &'static [u8]) -> Option<Effect> {
        for effect in self.effects.into_iter() {
            if effect.name == name {
                return Some(effect);
            }
        }

        return None;
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
    fn get_next_shader(&self) -> Shader {
        return match self.find_shader(b"R2D2") {
            None => return self.shaders.get(0).unwrap(),
            Some(shader) => {
                return shader;
            }
        }
    }

    // Returns a random effect
    fn get_next_effect(&self) -> Effect {
        let idx = rand() % self.effects.size() as u64;
        return self.effects.get(idx as usize).unwrap();
    }

    pub fn init(&mut self) {
        self.driver.init();

        // Initialize the contexts
        for node_id in 0 .. UNITS {
            self.contexts[node_id].node_id = node_id;
            self.contexts[node_id].total_nodes = UNITS;
            self.contexts[node_id].initialized = false;
        }

        // Select an effect
        self.effect = Some(self.get_next_effect());

        // Select a shader
        self.shader = self.find_shader(b"Medbay");
    }

    pub fn transition_to(&mut self, next_shader: Shader) {
        self.next_shader = Some(next_shader);
        
        // Randomize each hexagon unit
        for node_id in 0 .. UNITS {
            self.contexts[node_id].initialized = false;
        }

        // Randomize the next effect
        self.effect = Some(self.get_next_effect());

        // Set the transition start time
        self.transition_start = nanos();
        self.state = PixelState::Transitioning;


    }

    pub fn system_loop(&mut self) {
        let time = (nanos() - self.transition_offset);
        let elapsed_ms = time / teensycore::MS_TO_NANO;

        if time > self.target {
            let shader = self.shader.as_mut().unwrap();
            let effect = self.effect.as_mut().unwrap();

            match self.state {
                PixelState::Transitioning => {
                    
                    if time > (self.transition_start + TRANSITION_TIME * MS_TO_NANO) {
                        // We have arrived
                        self.state = PixelState::MainSequence;
                        self.shader = self.next_shader;
                        self.transition_offset = self.transition_start;
                    } else {
                        // We will interpolate from the snapshot of the last known colors
                        // into the computed effect of the next color. And once
                        // we've iterated the correct amount of time, we will
                        // swap next_shader with shader.
                        for node_id in 0 .. UNITS {
                            let mut ctx = self.contexts[node_id];
                            let next_shader = self.next_shader.as_mut().unwrap();
                            let transition_time_elapsed = (time - self.transition_start) / MS_TO_NANO;
                            let (effect_time, next_context) = effect.process(&mut ctx, transition_time_elapsed);
                            let time_t = ((effect_time as f64 / 100.0) * next_shader.total_time as f64) as u64;
                            let next_color = next_shader.get_color(time_t);
                            self.contexts[node_id] = next_context;
                            
                            let color = rgb(
                                interpolate(self.color_buffer[node_id].r as u32, next_color.r as u32, transition_time_elapsed, TRANSITION_TIME) as u8,
                                interpolate(self.color_buffer[node_id].g as u32, next_color.g as u32, transition_time_elapsed, TRANSITION_TIME) as u8,
                                interpolate(self.color_buffer[node_id].b as u32, next_color.b as u32, transition_time_elapsed, TRANSITION_TIME) as u8,
                            ).as_hex();

                            for pixel_id in 0 .. LEDS_PER_UNIT {
                                self.driver.set_color(node_id * LEDS_PER_UNIT + pixel_id, color);
                            }
                        }
                    }
                },

                PixelState::MainSequence |
                PixelState::Loading => {

                    // For each hexagon node
                    for node_id in 0 .. UNITS {
                        let mut ctx = self.contexts[node_id];
                        let (effect_time, next_context) = effect.process(&mut ctx, elapsed_ms);
                        let time_t = (( effect_time as f64 / 100.0) * shader.total_time as f64) as u64;
                        self.color_buffer[node_id] = shader.get_color(time_t);
                        let color = self.color_buffer[node_id].as_hex();

                        // Commit any updates to context that we should be registering
                        self.contexts[node_id] = next_context;

                        // Render the color for each unit in this node
                        for pixel_id in 0 .. LEDS_PER_UNIT {
                            self.driver.set_color(node_id * LEDS_PER_UNIT + pixel_id, color);
                        }
                    }
                },
            }

            self.driver.flush();
        }
    }

    pub fn ready(&mut self) {
        if !self.ready {
            self.ready = true;
            self.transition_to(self.get_next_shader());
        }
    }

}