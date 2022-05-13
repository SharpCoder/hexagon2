use crate::pixel_engine::effect::Effect;
use teensycore::clock::uNano;
use teensycore::{system::vector::*, vector, math::rand};

const TIME: uNano = 4200;

fn nano_rand() -> uNano {
    return rand() as uNano;
}

pub fn initialize_effects<'a>() -> Vector<Effect> {
    return vector!(

        Effect::new(b"Distributed")
            .with_initializer(|_, ctx| {
                let mut next_ctx = ctx.clone();
                let step = TIME / ctx.total_nodes;
                next_ctx.offset = (ctx.node_id * step) as uNano;
                return next_ctx;
            })
            .transition_to(100, TIME)
            .build(),


        Effect::new(b"Randomized")
            .with_max_color_segments(3)
            .with_initializer(|_, ctx| {
                let mut next_ctx = ctx.clone();
                next_ctx.offset = nano_rand() % TIME;
                return next_ctx;
            })
            .transition_to(100, TIME)
            .build(),

        // Effect::new(b"Alternate")
        //     .with_initializer(|_, ctx| {
        //         let mut next_ctx = ctx.clone();
        //         if ctx.node_id % 2 == 0 {
        //             next_ctx.offset = TIME / 2;
        //         } else {
        //             next_ctx.offset = 0;
        //         }

        //         return next_ctx;
        //     })
        //     .transition_to(100, TIME)
        //     .build(),

        Effect::new(b"Grouped")
            .with_max_color_segments(3)
            .with_initializer(|fx, ctx| {
                let mut next_ctx = ctx.clone();
                if  ctx.node_id == 0 || rand() % 2 == 0 {
                    fx.regs[1] = nano_rand() as i32;
                }
                next_ctx.offset = fx.regs[1]  as uNano % TIME;
                return next_ctx;
            })
            .transition_to(100, TIME)
            .transition_to(0, TIME)
            .transition_to(100, TIME)
            .build(),
            
        Effect::new(b"Surprise")
            .with_max_color_segments(3)
            .with_initializer(|_, ctx| {
                let mut next_ctx = ctx.clone();
                next_ctx.offset = nano_rand() % TIME / 2;
                return next_ctx;
            })
            .transition_to(50, TIME / 2)
            .transition_to(75, TIME / 3)
            .transition_to(100, TIME / 4)
            .build()
    );
}