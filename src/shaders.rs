use crate::pixel_engine::color::*;
use crate::pixel_engine::shader::*;
use teensycore::{system::vector::*, vector};
use teensycore::clock::uNano;

const TIME: uNano = 1000;

pub fn initialize_shaders<'a>() -> Vector<Shader> {
    return vector!(
        // Shader::new(b"Lunar")
        //     .with_color(rgb(199, 7, 2))
        //     .transition_to(rgb(255, 7, 2), TIME)
        //     .transition_to(rgb(255, 84, 2), TIME)
        //     .transition_to(rgb(199, 7, 2), TIME)
        //     .build(),

        // Shader::new(b"Independence")
        //     .as_wifi_only()
        //     .with_color(rgb(255, 0, 0))
        //     .transition_to(rgb(255,255,255), TIME)
        //     .transition_to(rgb(0, 0, 255), TIME)
        //     .transition_to(rgb(255, 0, 0), TIME)
        //     .build(),

        Shader::new(b"Medbay")
            .with_color(rgb(0, 255, 0))
            .transition_to(rgb(0,0,255), TIME)
            .transition_to(rgb(0, 255, 0), TIME)
            .build(),



        
        Shader::new(b"Honeycomb")
            .with_color(rgb(255, 180, 0)) // Orange
            .transition_to(rgb(255, 0, 0), TIME) // Red
            .transition_to(rgb(255, 180, 0), TIME) // Orange
            .build(),



        Shader::new(b"80SciFi")
            .as_disabled()
            .with_color(rgb(0, 145, 255)) // Tron Light Blue
            .transition_to(rgb(106, 0, 255), TIME) // Jazzersize Purple
            .transition_to(rgb(255, 0, 204), TIME) // Pink
            .transition_to(rgb(255,255,0), TIME) // Yellow
            .transition_to(rgb(0, 145, 255), TIME) // Tron Light Blue
            .build(),

        // Shader::new(b"R2D2")
        //     .with_color(rgb(0, 0, 255))
        //     // .transition_to(rgb(128, 128, 128), TIME/10)
        //     // .transition_to(rgb(0, 0, 255), TIME)
        //     .transition_to(rgb(255, 0, 0), TIME/5)
        //     .transition_to(rgb(0, 0, 255), TIME)
        //     .build(),

        Shader::new(b"Valentines")
            .with_color(rgb(255, 0, 0))
            .transition_to(rgb(50, 0, 255), TIME)
            .transition_to(rgb(255, 0, 0), TIME)
            .build(),

        // Shader::new(b"Shire")
        //     .as_wifi_only()
        //     .with_color(rgb(0, 255, 0))
        //     .transition_to(rgb(255, 255, 255), TIME)
        //     .transition_to(rgb(0, 255, 0), TIME)
        //     .build(),

        // Shader::new(b"Halloween")
        //     .as_wifi_only() 
        //     .with_color(rgb(255, 64, 0))
        //     .transition_to(rgb(255,0,0), TIME)
        //     .transition_to(rgb(0,0,0), TIME)
        //     .transition_to(rgb(0, 0, 0), TIME)
        //     .transition_to(rgb(255, 64, 0), TIME)
        //     .build(),

        // Shader::new(b"Xmas")
        //     .as_wifi_only()
        //     .with_color(rgb(255, 0, 0))
        //     .transition_to(rgb(0,255,0), TIME)
        //     .transition_to(rgb(255,0,0), TIME)
        //     .build(),

        // Shader::new(b"Mars")
        //     .as_disabled()
        //     .with_color(rgb(255, 0, 0))
        //     .transition_to(rgb(232,64,0), TIME)
        //     .transition_to(rgb(255,0,0), TIME)
        //     .build(),

        // Shader::new(b"Diwali")
        //     .as_wifi_only()
        //     .with_color(rgb(255,0,0))
        //     .transition_to(rgb(0, 161, 94), TIME)
        //     .transition_to(rgb(252, 210, 0), TIME)
        //     .transition_to(rgb(255,0,0), TIME)
        //     .transition_to(rgb(255,0,0), TIME)
        //     .build(),
        
        Shader::new(b"Dinosaur")
            .with_color(rgb(0, 0, 255))
            .transition_to(rgb(0, 255, 0), TIME)
            .transition_to(rgb(64, 0, 148), TIME)
            .transition_to(rgb(0, 0, 255), TIME)
            .set_segment_count(2) // Override segment count because colors are dupicated
            .build(),
        
        Shader::new(b"Rainbow")
            .with_color(rgb(255,0, 0))
            .transition_to(rgb(0,0,255), TIME)
            .transition_to(rgb(0,255,0), TIME)
            .transition_to(rgb(255,0,0), TIME)
            .build(),

        // Shader::new(b"Birthday")
        //     .as_wifi_only()
        //     .with_color(rgb(255, 0, 70))
        //     // .transition_to(rgb(124, 142, 208), TIME)
        //     .transition_to(rgb(255, 213, 0), TIME)
        //     .transition_to(rgb(77, 255, 0), TIME)
        //     .transition_to(rgb(255, 0, 70), TIME)
        //     .build(),


        // Shader::new(b"Pride")
        //     .with_color(rgb(255, 0, 0))
        //     .transition_to(rgb(255, 60, 0), TIME)
        //     .transition_to(rgb(255, 200, 0), TIME)
        //     .transition_to(rgb(0, 255, 0), TIME)
        //     .transition_to(rgb(0, 0, 255), TIME)
        //     .transition_to(rgb(60, 0, 255), TIME)
        //     .transition_to(rgb(255, 0, 0), TIME)
        //     .build(),

        Shader::new(b"Jupiter")
            .as_disabled()
            .with_color(rgb(49, 0, 51))
            .transition_to(rgb(225,35,0), TIME)
            .transition_to(rgb(255,0,0), TIME)
            .transition_to(rgb(49, 0, 51), TIME)
            .build(),

        
        Shader::new(b"Neptune")
            .with_color(rgb(0,0,188))
            .transition_to(rgb(0, 35, 194), TIME)
            .transition_to(rgb(0, 255, 183), TIME)
            .transition_to(rgb(60, 0, 255), TIME)
            .transition_to(rgb(0,0,188), TIME)
            .build(),

        Shader::new(b"R2D2")
            .with_color(rgb(0, 0, 255)) // Blue
            .transition_to(rgb(255, 0, 0), TIME) // Red
            .transition_to(rgb(0, 0, 255), TIME) // Blue
            .build(),

        Shader::new(b"RetroFuturistic")
            .with_color(rgb(255, 0, 173))
            .transition_to(rgb(255, 0, 0), TIME)
            .transition_to(rgb(115, 6, 121), TIME)
            .transition_to(rgb(4, 110, 106), TIME)
            .transition_to(rgb(255, 255, 0), TIME)
            .transition_to(rgb(255, 0, 173), TIME)
            .build()

        // Shader::new(b"Thanksgiving")
        //     .as_wifi_only()
        //     .with_color(rgb(255, 98, 0))
        //     .transition_to(rgb(255, 0, 0), TIME)
        //     .transition_to(rgb(255, 98, 0), TIME)
        //     .build(),

        // Shader::new(b"Pokemon")
        //     .as_wifi_only()
        //     .with_color(rgb(255, 255, 0))
        //     .transition_to(rgb(0, 0, 255), TIME)
        //     .transition_to(rgb(255, 255, 0), TIME)
        //     .build()


        // Shader::new(b"DoctorWho")
        //     .as_wifi_only()
        //     .with_color(rgb(0, 0, 255))
        //     .transition_to(rgb(0, 0, 255), TIME)
        //     .transition_to(rgb(255, 255, 255), TIME)
        //     .transition_to(rgb(0, 0, 255), TIME)
        //     .transition_to(rgb(0, 0, 255), TIME)
        //     .build(),

        // Shader::new(b"Pirate")
        //     .as_wifi_only()
        //     .with_color(rgb(255, 0, 0))
        //     .transition_to(rgb(255, 255, 255), TIME)
        //     .transition_to(rgb(255, 0, 0), TIME)
        //     .build(),

        // Shader::new(b"BattlestarGalactica")
        //     .as_wifi_only()
        //     .with_color(rgb(255, 255, 255))
        //     .transition_to(rgb(255, 0, 0), TIME)
        //     .transition_to(rgb(200, 200, 200), TIME)
        //     .transition_to(rgb(255, 0, 0), TIME)
        //     .transition_to(rgb(255, 255, 255), TIME)
        //     .build(),
        
    );
}