
use teensycore::clock::*;
use teensycore::math::rand;
use teensycore::system::vector::Vector;
use crate::pixel_engine::color::*;
use crate::pixel_engine::shader::*;
use crate::pixel_engine::effect::*;
use crate::pixel_engine::context::*;
use crate::drivers::ws2812::*;
use crate::shaders::initialize_shaders;

const UNITS: usize = 7;
const LEDS_PER_UNIT: usize = 3;
const LEDS: usize = UNITS * LEDS_PER_UNIT;
pub struct PixelTask {
    shader: Option<Shader>,
    shaders: Vector<Shader>,
    contexts: [Context; UNITS],
    effect: Effect,
    driver: WS2812Driver<LEDS>,
    target: u64,
}

impl PixelTask {
    pub fn new() -> Self {

        return PixelTask {
            target: 0,
            shader: None,
            shaders: initialize_shaders(),
            driver: WS2812Driver::<LEDS>::new(
                18, // pin
            ),
            contexts: [Context::empty(); UNITS],
            effect: Effect::new(b"Infection")
                .with_initializer(|ctx| {
                    let mut next_ctx = ctx.clone();
                    // next_ctx.offset = rand() % 2000;
                    next_ctx.offset = ((ctx.node_id as f64 / ctx.total_nodes as f64) * 4000.0) as u64;
                    // let origin = ctx.total_nodes / 2;
                    // if (ctx.node_id + origin) > origin {
                    //     next_ctx.offset = (ctx.total_nodes - ctx.node_id) as u64 * 200;
                    // } else {
                    //     next_ctx.offset = (ctx.node_id + origin) as u64 * 200;
                    // }
                    // next_ctx.offset += rand() % 250;
                    return next_ctx;
                })
                .transition_to(100, 4000)
                // .transition_to_and_hold(100, 500)
                // .transition_to(100, 2000)
                // .transition_to(0, 4000)
                .build(),
        };   
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
        for shader in self.shaders.into_iter() {
            if shader.name == b"Birthday" {
                self.shader = Some(shader);
            }
        }
    }

    pub fn system_loop(&mut self) {
        let time = nanos();
        let elapsed_ms = time / teensycore::MS_TO_NANO;
        if time > self.target {

            match self.shader.as_mut() {
                None => {},
                Some(shader) => {
                    for node_id in 0 .. UNITS {
                        let mut ctx = self.contexts[node_id];
                        let (time_t, next_context) = self.effect.process(&mut ctx, elapsed_ms);
                        let time_t = (( time_t as f64 / 100.0) * shader.total_time as f64) as u64;
                        
                        self.contexts[node_id] = next_context;
                        for pixel_id in 0 .. LEDS_PER_UNIT {
                            // let color = self.shader.get_color((time + 350 * teensycore::MS_TO_NANO * node_id as u64) / teensycore::MS_TO_NANO).as_hex();
                            let color = shader.get_color(time_t).as_hex();
                            self.driver.set_color(node_id * LEDS_PER_UNIT + pixel_id, color);
                        }
                    }
            
                    self.driver.flush();
                    self.target = nanos() + 7 * teensycore::MS_TO_NANO;
                }
            }
        }

    }

}