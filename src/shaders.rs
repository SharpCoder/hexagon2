use crate::pixel_engine::color::*;
use crate::pixel_engine::shader::*;
use teensycore::{system::vector::*, vector};

const TIME: u64 = 1500;

pub fn initialize_shaders<'a>() -> Vector<Shader> {
    return vector!(
        Shader::new(b"Lunar")
            .with_color(rgb(199, 7, 2))
            .transition_to(rgb(255, 7, 2), TIME)
            .transition_to(rgb(255, 84, 2), TIME)
            .transition_to(rgb(199, 7, 2), TIME)
            .build(),

        Shader::new(b"Independence")
            .with_color(rgb(255, 0, 0))
            .transition_to(rgb(255,255,255), TIME)
            .transition_to(rgb(0, 0, 255), TIME)
            .transition_to(rgb(255, 0, 0), TIME)
            .build(),

        Shader::new(b"Medbay")
            .with_color(rgb(0, 255, 0))
            .transition_to(rgb(0,0,255), TIME)
            .transition_to(rgb(0, 255, 0), TIME)
            .build(),

        Shader::new(b"Halloween")
            .with_color(rgb(255, 64, 0))
            .transition_to(rgb(255,0,0), TIME)
            .transition_to(rgb(0,0,0), TIME)
            .transition_to(rgb(255, 64, 0), TIME)
            .build(),

        Shader::new(b"SpaceX")
            .with_color(rgb(0, 0, 0))
            .transition_to(rgb(255,255,255), TIME)
            .transition_to(rgb(0,0,0), TIME)
            .build(),

        Shader::new(b"Xmas")
            .with_color(rgb(255, 0, 0))
            .transition_to(rgb(0,255,0), TIME)
            .transition_to(rgb(255,0,0), TIME)
            .build(),

        Shader::new(b"Mars")
            .with_color(rgb(255, 0, 0))
            .transition_to(rgb(232,64,0), TIME)
            .transition_to(rgb(255,0,0), TIME)
            .build(),

        Shader::new(b"Duna")
            .with_color(rgb(255, 0, 0))
            .transition_to(rgb(232,64,0), TIME)
            .transition_to(rgb(255,255,255), TIME)
            .transition_to(rgb(232,64,0), TIME)
            .transition_to(rgb(255,0,0), TIME)
            .transition_to(rgb(255,0,0), TIME)
            .build(),

        Shader::new(b"Earth")
            .with_color(rgb(0,255,0))
            .transition_to(rgb(0, 0, 220), TIME)
            .transition_to(rgb(10, 82, 6), TIME)
            .transition_to(rgb(25,49,15), TIME)
            .transition_to(rgb(0,255,0), TIME)
            .build(),

        Shader::new(b"Diwali")
            .with_color(rgb(255,0,0))
            .transition_to(rgb(0, 161, 94), TIME)
            .transition_to(rgb(252, 210, 0), TIME)
            .transition_to(rgb(255,0,0), TIME)
            .transition_to(rgb(255,0,0), TIME)
            .build(),

        Shader::new(b"Virus")
            .with_color(rgb(255, 0, 0))
            .transition_to(rgb(255, 255, 255), TIME)
            .transition_to(rgb(255,0,0), TIME)
            .build(),
        
        Shader::new(b"Dinosaur")
            .with_color(rgb(0, 255, 0))
            .transition_to(rgb(31, 0, 99), TIME)
            .transition_to(rgb(0, 255, 0), TIME)
            .build(),
        
        Shader::new(b"Stars")
            .with_color(rgb(4, 0, 82))
            .transition_to(rgb(5, 59, 93), TIME)
            .transition_to(rgb(158, 150, 0), TIME)
            .transition_to(rgb(4, 0, 82), TIME)
            .build(),
        
        Shader::new(b"Rainbow")
            .with_color(rgb(255,0, 0))
            .transition_to(rgb(0,0,255), TIME)
            .transition_to(rgb(0,255,0), TIME)
            .transition_to(rgb(255,0,0), TIME)
            .build(),

        Shader::new(b"Birthday")
            .with_color(rgb(255, 0, 70))
            .transition_to(rgb(124, 142, 208), TIME)
            .transition_to(rgb(255, 213, 0), TIME)
            .transition_to(rgb(77, 255, 0), TIME)
            .transition_to(rgb(255, 0, 70), TIME)
            .build(),

        Shader::new(b"JWST")
            .with_color(rgb(255, 60, 0))
            .transition_to(rgb(0, 0, 0), TIME)
            .transition_to(rgb(255, 60, 0), TIME)
            .build(),

        Shader::new(b"Pride")
            .with_color(rgb(255, 0, 0))
            .transition_to(rgb(255, 60, 0), TIME)
            .transition_to(rgb(255, 200, 0), TIME)
            .transition_to(rgb(0, 255, 0), TIME)
            .transition_to(rgb(0, 0, 255), TIME)
            .transition_to(rgb(60, 0, 255), TIME)
            .transition_to(rgb(255, 0, 0), TIME)
            .build()
        
    );
}