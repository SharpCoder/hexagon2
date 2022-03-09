use crate::pixel_engine::effect::Effect;
use teensycore::{system::vector::*, vector, math::rand};

const TIME: u64 = 2500;

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
                let step = TIME / ctx.total_nodes;
                next_ctx.offset = (node_id * step) as u64;
                return next_ctx;
            })
            .transition_to(100, TIME)
            .transition_to(100, TIME)
            .build(),

        Effect::new(b"Distributed")
            .with_initializer(|ctx| {
                let mut next_ctx = ctx.clone();
                let step = TIME / ctx.total_nodes;
                next_ctx.offset = (ctx.node_id * step) as u64;
                return next_ctx;
            })
            .transition_to(100, TIME)
            .build(),

        Effect::new(b"Randomized")
            .with_initializer(|ctx| {
                let mut next_ctx = ctx.clone();
                next_ctx.offset = rand() % TIME / 2;
                return next_ctx;
            })
            .transition_to(100, TIME)
            .build(),

        Effect::new(b"DanceParty")
            .with_initializer(|ctx| {
                let mut next_ctx = ctx.clone();
                next_ctx.offset = rand() % 4000;
                return next_ctx;
            })
            .transition_to(100, 4000)
            .transition_to(0, 2000)
            .transition_to(100, 3000)
            .transition_to(0, 1750)
            .transition_to(100, 2500)
            .build(),

        Effect::new(b"Surprise")
            .with_initializer(|ctx| {
                let mut next_ctx = ctx.clone();
                next_ctx.offset = rand() % 2000;
                return next_ctx;
            })
            .transition_to(50, 2000)
            .transition_to(75, 1000)
            .transition_to(100, 800)
            .build(),

        Effect::new(b"Grouped")
            .with_initializer(|ctx| {
                let mut next_ctx = ctx.clone();
                next_ctx.offset = (rand() % 3) as u64 * 1000;
                return next_ctx;
            })
            .transition_to(100, 3000)
            .build(),

        Effect::new(b"Grouped2")
            .with_initializer(|ctx| {
                let mut next_ctx = ctx.clone();
                next_ctx.offset = (rand() % 4) as u64 * 500;
                return next_ctx;
            })
            .transition_to(100, 3000)
            .build(),

        Effect::new(b"Wave")
            .with_initializer(|ctx| {
                let mut next_ctx = ctx.clone();
                let id = ctx.node_id + 1;
                let step = TIME as f32 / ctx.total_nodes as f32;
                let max = (step * id as f32) as u64;

                next_ctx.offset = rand() % max;
                return next_ctx;
            })
            .transition_to(100, TIME)
            .build()
    );
}