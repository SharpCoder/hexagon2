use teensycore::*;
use teensycore::clock::*;
use teensycore::math::rand;
use teensycore::system::str::Str;
use teensycore::system::str::StringOps;
use teensycore::system::vector::Array;
use teensycore::system::vector::Vector;
use crate::date_time::DateTime;
use crate::get_shader_configs;
use crate::get_tranasition_delay;
use crate::shaders::*;
use crate::effects::*;
use crate::pixel_engine::color::*;
use crate::pixel_engine::math::interpolate;
use crate::pixel_engine::shader::*;
use crate::pixel_engine::effect::*;
use crate::pixel_engine::context::*;
use crate::drivers::ws2812::*;

const LEDS_PER_UNIT: usize = 3;
const LEDS: usize = crate::HEX_UNITS * LEDS_PER_UNIT;
const TRANSITION_TIME: uNano = 500; // ms

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
    contexts: [Context; crate::HEX_UNITS],
    effect: Option<Effect>,
    effects: Vector<Effect>,
    driver: WS2812Driver<LEDS>,
    target: uNano,
    day_target: uNano,

    /// The day on which we last randomized the sequence
    day_processed: uNano,
    cycles: u64,
    
    // Randomize every couple hours
    randomize_target: uNano,
    ready: bool,
    color_buffer: [Color; crate::HEX_UNITS],
    transition_start: uNano,
    transition_offset: uNano,
    cycle_offset: uNano,
}

impl PixelTask {
    pub fn new() -> Self {

        return PixelTask {
            state: PixelState::Loading,
            target: 0,
            day_target: 0,
            randomize_target: 0,
            day_processed: 0,
            transition_start: 0,
            transition_offset: 0,
            cycle_offset: 0,
            cycles: 0,
            ready: false,
            shader: None,
            effect: None,
            next_shader: None,
            shaders: initialize_shaders(),
            effects: initialize_effects(),
            driver: WS2812Driver::<LEDS>::new(
                18, // pin
            ),
            color_buffer: [Color::blank(); crate::HEX_UNITS],
            contexts: [Context::empty(); crate::HEX_UNITS],
        };   
    }

    #[allow(dead_code)]
    fn find_effect(&self, name: &'static [u8]) -> Option<Effect> {
        for effect in self.effects.into_iter() {
            if effect.name == name {
                return Some(effect);
            }
        }

        return None;
    }

    fn find_shader(&self, name: &Str) -> Option<Shader> {
        for shader in self.shaders.into_iter() {
            let mut shader_name = Str::new();
            shader_name.append(shader.name);

            if name.contains(&shader_name) {

                shader_name.drop();
                return Some(shader);
            }

            shader_name.drop();
        }

        return None;

    }

    fn cycle_next_shader(&mut self) {
        let idx = (self.cycles / 3) as usize % self.shaders.size();
        let shader = self.shaders.get(idx).unwrap();
        self.transition_to(shader);
    }


    // Evaluate which shader to select based on
    // world information.
    fn get_next_shader(&self) -> Shader {
        // return self.find_shader(&str!(b"R2D2")).unwrap();

        // If we have WIFI access, use the shader configs downloaded from the internet
        if crate::USE_WIFI {
            let appropriate_shader = get_shader_configs().get_shader(crate::get_world_time());
            match self.find_shader(&appropriate_shader) {
                None => return self.shaders.get(0).unwrap(),
                Some(shader) => { 
                    if shader.disabled {
                         return self.get_next_shader();
                    }
                    return shader; 
                }
            }
        } else {
            // Otherwise, there is no wifi. Return any random shader.
            let idx = rand() % self.shaders.size() as u64;
            let next_shader = self.shaders.get(idx as usize).unwrap();
            if next_shader.wifi_only || next_shader.disabled {
                return self.get_next_shader();
            } else {
                return next_shader;
            }
        }
    }

    // Returns a random effect
    fn get_next_effect(&self, shader: &Shader) -> Effect {
        // if shader.total_segments < 3 {
        //     return self.find_effect(b"Randomized").unwrap();
        // } else {
            // return self.find_effect(b"Explosion").unwrap();
        // }

        // Select an appropriate effect to match the shader
        let idx = rand() % self.effects.size() as u64;
        let next_effect = self.effects.get(idx as usize).unwrap();
        if next_effect.disabled ||  next_effect.max_color_segments.unwrap_or(usize::MAX) < shader.total_segments || next_effect.min_hex_units.unwrap_or(0) > crate::HEX_UNITS {
            return self.get_next_effect(shader);
        } else {
            return next_effect;
        }
    }

    pub fn init(&mut self) {
        self.driver.init();

        // Initialize the contexts
        for node_id in 0 .. crate::HEX_UNITS {
            self.contexts[node_id].node_id = node_id as uNano;
            self.contexts[node_id].total_nodes = crate::HEX_UNITS as uNano;
            self.contexts[node_id].initialized = false;
        }

        // Set the next day processing target
        self.day_target = nanos() + (S_TO_NANO * 60 * 30);

        // Select a shader
        self.shader = self.find_shader(&str!(b"Medbay"));
        
        // Select an effect
        self.effect = Some(self.find_effect(b"Randomized").unwrap());
    }

    pub fn transition_to(&mut self, next_shader: Shader) {
        self.next_shader = Some(next_shader);
        
        // Randomize each hexagon unit
        for node_id in 0 .. crate::HEX_UNITS {
            self.contexts[node_id].initialized = false;
            self.contexts[node_id].node_id = node_id as uNano;
        }

        // Randomize the next effect
        self.effect = Some(self.get_next_effect(&next_shader));

        // Set the transition start time
        self.transition_start = nanos();
        self.transition_offset = 0;
        self.state = PixelState::Transitioning;
    }

    pub fn randomize(&mut self) {
        self.randomize_target = nanos() + get_tranasition_delay();
        self.transition_to(self.get_next_shader());
    }

    pub fn system_loop(&mut self) {
        // for node_id in 0 .. crate::HEX_UNITS {

        //     // Render the color for each unit in this node
        //     for pixel_id in 0 .. LEDS_PER_UNIT {
        //         self.driver.set_color(node_id * LEDS_PER_UNIT + pixel_id, 0x0000FF);
        //     }
        // }

        // if self.cycles < 3 {
        //     self.cycles += 1;
        // }
        // self.driver.flush();

        // blink_accumulate();
        
        // return;

        let time = nanos() - self.transition_offset;
        let cycle_time = (time - self.cycle_offset) / teensycore::MS_TO_NANO;
        let elapsed_ms = time / teensycore::MS_TO_NANO;
        let mut should_cycle = false;

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
                        for node_id in 0 .. crate::HEX_UNITS {
                            let mut ctx = self.contexts[node_id];
                            let next_shader = self.next_shader.as_mut().unwrap();
                            let transition_time_elapsed = (time - self.transition_start) / MS_TO_NANO;
                            let (effect_time, next_context) = effect.process(&mut ctx, transition_time_elapsed);
                            let time_t = ((effect_time as f64 / 100.0) * next_shader.total_time as f64) as uNano;
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

                    if cycle_time > effect.total_time {
                        self.cycles += 1;
                        self.cycle_offset = nanos() - self.transition_offset;

                        // If cycle mode, then advance to the next color theme
                        if crate::CYCLE_MODE && self.cycles % 3 == 0 {
                            should_cycle = true;
                        }
                    }

                    // For each hexagon node
                    for node_id in 0 .. crate::HEX_UNITS {
                        let mut ctx = self.contexts[node_id];
                        let (effect_time, next_context) = effect.process(&mut ctx, elapsed_ms);
                        let time_t = (( effect_time as f64 / 100.0) * shader.total_time as f64) as uNano;
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


            match self.state {
                PixelState::MainSequence => {
                    // day_target is only valid if we can sync world_clock with the wifi
                    // which doesn't happen if WIFI is disabled.
                    if crate::USE_WIFI && nanos() > self.day_target {
                        // Check if we need to recalculate transition
                        let datetime = DateTime::now();
                        if self.day_processed != datetime.days && datetime.hour >= 6 {
                            self.day_processed = datetime.days;
                            self.randomize();
                        }
                        self.day_target = nanos() + S_TO_NANO;// (S_TO_NANO * 60 * 30);
                    } else if nanos() > self.randomize_target {
                        self.randomize();
                    }
                },
                _ => {},
            }


            self.driver.flush();
        }

        if should_cycle {
            self.cycle_next_shader();
        }
    }

    pub fn ready(&mut self) {
        if !self.ready {
            self.ready = true;
            self.randomize();
        }
    }

}