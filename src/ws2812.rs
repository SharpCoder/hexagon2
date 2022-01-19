pub mod shader;

use crate::drivers::ws2812::*;
use teensycore::clock::*;

use self::shader::*;

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


pub struct WS2812Task { 
    target: u64,
    driver: WS2812Driver<LEDS>,
    shader: ActiveShader,
    contexts: [ShaderContext; LEDS],
}

#[derive(Copy, Clone)]
pub enum ActiveShader {
    Basic,
    Xmas,
    Constrained,
    AudioEqualizer,
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

impl WS2812Task {

    pub fn set_shader(&mut self, shader: ActiveShader) {
        let active_shader = get_shader(shader);
        for i in 0 .. LEDS {
            self.contexts[i] = active_shader.init(self.contexts[i]);
        }
    }
}

impl teensycore::Task for WS2812Task {

    fn new() -> WS2812Task {
        return WS2812Task { 
            shader: ActiveShader::Basic,
            target: 0,
            contexts: [ShaderContext::new(0, LEDS); LEDS],
            driver: WS2812Driver::<LEDS>::new(
                18, // pin
            ),
        };
    }

    fn init(&mut self) {
        for idx in 0 .. LEDS {
            self.contexts[idx].node_id = idx;
            self.contexts[idx].total_nodes = LEDS;
        }
        self.set_shader(self.shader);
        self.system_loop();
    }

    fn system_loop(&mut self) {
        if nanos() > self.target {
            let active_shader = get_shader(self.shader);
            for i in 0 .. LEDS {
                self.contexts[i] = active_shader.update(self.contexts[i]);
                self.driver.set_color(i, self.contexts[i].color);
            }

            self.driver.flush();
            self.target = nanos() + teensycore::MS_TO_NANO * 18;
        }
    }
}