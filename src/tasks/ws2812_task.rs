mod shader;

use crate::Task;
use crate::drivers::ws2812::*;
use crate::clock::*;

use self::shader::*;

const LEDS: usize = 1;
  
pub struct WS2812Task { 
    target: u64,
    driver: WS2812Driver<LEDS>,
    shader: ActiveShader,
}

#[derive(Copy, Clone)]
pub enum ActiveShader {
    Basic,
    Xmas,
}

static mut BASIC_SHADER: BasicShader = BasicShader::new();
static mut XMAS_SHADER: XmasShader = XmasShader::new();

fn get_shader(shader: ActiveShader) -> &'static mut dyn Shader::<LEDS> {
    return match shader {
        ActiveShader::Basic => unsafe { &mut BASIC_SHADER }, 
        ActiveShader::Xmas => unsafe { &mut XMAS_SHADER },
    };
}

impl Task<WS2812Task> for WS2812Task {

    fn new() -> WS2812Task {
        return WS2812Task { 
            shader: ActiveShader::Xmas,
            target: 0,
            driver: WS2812Driver::<LEDS>::new(
                18, // pin
            ),
        };
    }

    fn init(&mut self) {
        self.driver.set_color(0, 0xFF0000);
    }

    fn system_loop(&mut self) {
        if nanos() > self.target {
            let driver = &mut self.driver;
            let active_shader = get_shader(self.shader);

            for i in 0 .. LEDS {
                let context = ShaderContext {
                    node_id: i,
                    total_nodes: LEDS,
                    current_time: crate::clock::nanos(),
                    temperature: 0,
                };

                active_shader.update(driver, context);
            }
            self.target = nanos() + crate::MS_TO_NANO * 20;
        }
    }
}