use crate::pixel_engine::color::*;
use crate::pixel_engine::effect::Effect;
use teensycore::{system::vector::*, vector, math::rand};

const TIME: u64 = 3500;

pub fn initial_effects<'a>() -> Vector<Effect> {
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
                let step = (TIME as usize / ctx.total_nodes);
                next_ctx.offset = (node_id * step) as u64;
                return next_ctx;
            })
            .transition_to(100, TIME)
            .transition_to(100, TIME)
            .build(),

        Effect::new(b"Distributed")
            .with_initializer(|ctx| {
                let mut next_ctx = ctx.clone();
                let step = (TIME as usize / ctx.total_nodes);
                next_ctx.offset = (ctx.node_id * step) as u64;
                return next_ctx;
            })
            .transition_to(100, TIME)
            .build(),

        Effect::new(b"Randomized")
            .with_initializer(|ctx| {
                let mut next_ctx = ctx.clone();
                next_ctx.offset = (rand() % TIME as usize as u64 / 2);
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
            .transition_to(0, 1250)
            .transition_to(100, 1500)
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
            .build()
    );
}