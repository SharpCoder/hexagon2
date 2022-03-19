use crate::pixel_engine::color::*;
use crate::pixel_engine::shader::*;
use teensycore::{system::vector::*, vector};

const TIME: u64 = 1500;

pub fn initialize_shaders<'a>() -> Vector<Shader> {
    return vector!(
        Shader::new(b"Lunar")
            .as_wifi_only()
            .with_color(rgb(199, 7, 2))
            .transition_to(rgb(255, 7, 2), TIME)
            .transition_to(rgb(255, 84, 2), TIME)
            .transition_to(rgb(199, 7, 2), TIME)
            .build(),

        Shader::new(b"Independence")
            .as_wifi_only()
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
            .as_wifi_only() 
            .with_color(rgb(255, 64, 0))
            .transition_to(rgb(255,0,0), TIME)
            .transition_to(rgb(0,0,0), TIME)
            .transition_to(rgb(0, 0, 0), TIME)
            .transition_to(rgb(255, 64, 0), TIME)
            .build(),

        Shader::new(b"Xmas")
            .as_wifi_only()
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

        Shader::new(b"Diwali")
            .as_wifi_only()
            .with_color(rgb(255,0,0))
            .transition_to(rgb(0, 161, 94), TIME)
            .transition_to(rgb(252, 210, 0), TIME)
            .transition_to(rgb(255,0,0), TIME)
            .transition_to(rgb(255,0,0), TIME)
            .build(),
        
        Shader::new(b"Dinosaur")
            .with_color(rgb(0, 255, 0))
            .transition_to(rgb(31, 0, 99), TIME)
            .transition_to(rgb(0, 255, 0), TIME)
            .build(),
        
        Shader::new(b"Rainbow")
            .with_color(rgb(255,0, 0))
            .transition_to(rgb(0,0,255), TIME)
            .transition_to(rgb(0,255,0), TIME)
            .transition_to(rgb(255,0,0), TIME)
            .build(),

        Shader::new(b"Birthday")
            .as_wifi_only()
            .with_color(rgb(255, 0, 70))
            .transition_to(rgb(124, 142, 208), TIME)
            .transition_to(rgb(255, 213, 0), TIME)
            .transition_to(rgb(77, 255, 0), TIME)
            .transition_to(rgb(255, 0, 70), TIME)
            .build(),


        Shader::new(b"Pride")
            .as_wifi_only()
            .with_color(rgb(255, 0, 0))
            .transition_to(rgb(255, 60, 0), TIME)
            .transition_to(rgb(255, 200, 0), TIME)
            .transition_to(rgb(0, 255, 0), TIME)
            .transition_to(rgb(0, 0, 255), TIME)
            .transition_to(rgb(60, 0, 255), TIME)
            .transition_to(rgb(255, 0, 0), TIME)
            .build(),
        
        Shader::new(b"Starfleet")
            .with_color(rgb(0, 0, 255))
            .transition_to(rgb(255, 0, 0), TIME)
            .transition_to(rgb(255, 255, 0), TIME)
            .transition_to(rgb(0, 0, 255), TIME)
            .build(),


        Shader::new(b"R2D2")
            .with_color(rgb(0, 0, 255))
            .transition_to(rgb(255, 255, 255), TIME)
            .transition_to(rgb(0, 0, 255), TIME)
            .transition_to(rgb(255, 0, 0), TIME)
            .transition_to(rgb(0, 0, 255), TIME)
            .build(),

        Shader::new(b"Jupiter")
            .with_color(rgb(49, 0, 51))
            .transition_to(rgb(225,35,0), TIME)
            .transition_to(rgb(255,0,0), TIME)
            .transition_to(rgb(49, 0, 51), TIME)
            .build(),

        Shader::new(b"Valentines")  
            .as_wifi_only()
            .with_color(rgb(255, 0, 0))
            .transition_to(rgb(255, 0, 195), TIME)
            .transition_to(rgb(255, 255,255), TIME)
            .transition_to(rgb(255, 0, 195), TIME)
            .transition_to(rgb(255, 0, 0), TIME)
            .build(),

        Shader::new(b"Neptune")
            .with_color(rgb(0,0,188))
            .transition_to(rgb(0, 35, 194), TIME)
            .transition_to(rgb(0, 255, 183), TIME)
            .transition_to(rgb(60, 0, 255), TIME)
            .transition_to(rgb(0,0,188), TIME)
            .build(),

        Shader::new(b"RetroFuturistic")
            .with_color(rgb(255, 70, 173))
            .transition_to(rgb(255, 0, 0), TIME)
            .transition_to(rgb(115, 6, 121), TIME)
            .transition_to(rgb(4, 110, 106), TIME)
            .transition_to(rgb(255, 255, 0), TIME)
            .transition_to(rgb(255, 70, 173), TIME)
            .build(),

        Shader::new(b"Thanksgiving")
            .as_wifi_only()
            .with_color(rgb(255, 98, 0))
            .transition_to(rgb(255, 0, 0), TIME)
            .transition_to(rgb(255, 98, 0), TIME)
            .build(),

        Shader::new(b"Pokemon")
            .as_wifi_only()
            .with_color(rgb(255, 255, 0))
            .transition_to(rgb(0, 0, 255), TIME)
            .transition_to(rgb(255, 255, 0), TIME)
            .build(),

        Shader::new(b"Shire")
            .as_wifi_only()
            .with_color(rgb(0, 255, 0))
            .transition_to(rgb(0, 255, 0), TIME)
            .transition_to(rgb(255, 255, 255), TIME)
            .transition_to(rgb(0, 255, 0), TIME)
            .build(),

        Shader::new(b"DoctorWho")
            .as_wifi_only()
            .with_color(rgb(0, 0, 255))
            .transition_to(rgb(0, 0, 255), TIME)
            .transition_to(rgb(255, 255, 255), TIME)
            .transition_to(rgb(0, 0, 255), TIME)
            .transition_to(rgb(0, 0, 255), TIME)
            .build(),

        Shader::new(b"Pirate")
            .as_wifi_only()
            .with_color(rgb(255, 0, 0))
            .transition_to(rgb(255, 255, 255), TIME)
            .transition_to(rgb(255, 0, 0), TIME)
            .build(),

        Shader::new(b"BattlestarGalactica")
            .as_wifi_only()
            .with_color(rgb(255, 255, 255))
            .transition_to(rgb(255, 0, 0), TIME)
            .transition_to(rgb(200, 200, 200), TIME)
            .transition_to(rgb(255, 0, 0), TIME)
            .transition_to(rgb(255, 255, 255), TIME)
            .build(),

        Shader::new(b"80SciFi")
            .with_color(rgb(20,255,25))
            .transition_to(rgb(42,0,234), TIME)
            .transition_to(rgb(144,0,238), TIME)
            .transition_to(rgb(144,238,0), TIME)
            .transition_to(rgb(200,20,20), TIME)
            .transition_to(rgb(144,138,238), TIME)
            .transition_to(rgb(20,255,25), TIME)
            .build()
        
    );
}