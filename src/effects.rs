use crate::pixel_engine::effect::Effect;
use teensycore::clock::uNano;
use teensycore::{system::vector::*, vector, math::rand};

const TIME: uNano = 4500;

fn nano_rand() -> uNano {
    return rand() as uNano;
}

pub fn initialize_effects<'a>() -> Vector<Effect> {
    return vector!(
        Effect::new(b"Infection")
            .with_initializer(|ctx| {
                let mut next_ctx = ctx.clone();
                let origin = ctx.total_nodes / 2;
                let node_id;

                if ctx.node_id > origin {
                    node_id = ctx.total_nodes - ctx.node_id;
                } else {
                    node_id = ctx.node_id + origin;
                }

                let step = TIME / ctx.total_nodes as uNano;
                next_ctx.offset = (node_id * step) as uNano;
                return next_ctx;
            })
            .transition_to(100, TIME)
            .transition_to(100, TIME)
            .build(),

        Effect::new(b"Distributed")
            .with_initializer(|ctx| {
                let mut next_ctx = ctx.clone();
                let step = TIME / ctx.total_nodes;
                next_ctx.offset = (ctx.node_id * step) as uNano;
                return next_ctx;
            })
            .transition_to(100, TIME)
            .build(),

        Effect::new(b"Randomized")
            .with_initializer(|ctx| {
                let mut next_ctx = ctx.clone();
                next_ctx.offset = nano_rand() % TIME;
                return next_ctx;
            })
            .transition_to(100, TIME)
            .build(),

        Effect::new(b"DanceParty")
            .as_disabled()
            .with_initializer(|ctx| {
                let mut next_ctx = ctx.clone();
                next_ctx.offset = nano_rand() % TIME;
                return next_ctx;
            })
            .transition_to(100, TIME)
            .transition_to(0, TIME / 2)
            .transition_to(100, TIME / 3)
            .transition_to(0, TIME / 3)
            .transition_to(100, TIME / 2)
            .build(),

        Effect::new(b"Surprise")
            .with_initializer(|ctx| {
                let mut next_ctx = ctx.clone();
                next_ctx.offset = nano_rand() % TIME / 2;
                return next_ctx;
            })
            .transition_to(50, TIME / 2)
            .transition_to(75, TIME / 3)
            .transition_to(100, TIME / 4)
            .build(),


        Effect::new(b"Wave")
            .with_initializer(|ctx| {
                let mut next_ctx = ctx.clone();
                let id = ctx.node_id + 1;
                let step = TIME / ctx.total_nodes;
                let max = (step * id) as uNano;

                next_ctx.offset = (nano_rand() % max) as uNano;
                return next_ctx;
            })
            .transition_to(100, TIME)
            .build()
    );
}