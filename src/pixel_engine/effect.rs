use teensycore::mem::*;
use teensycore::clock::uNano;
use crate::pixel_engine::math::*;
use crate::pixel_engine::context::*;
#[derive(Copy, Clone)]
pub struct EffectNode {
    pub duration: uNano,
    pub target: u32,
    hold: bool,
    next: Option<*mut EffectNode>,
}

#[derive(Clone, Copy)]
pub struct Effect {
    pub name: &'static [u8],
    initializer: Option<fn(effect: &mut Effect, context: &Context) -> Context>,
    root: Option<*mut EffectNode>,
    pub total_time: uNano,
    pub disabled: bool,
    pub max_color_segments: Option<usize>,
    pub min_hex_units: Option<usize>,
    pub regs: [i32; 6],
}

impl Effect {
    pub fn new(name: &'static [u8]) -> Self {
        return Effect { 
            name: name,
            initializer: None,
            root: None,
            total_time: 0,
            max_color_segments: None,
            disabled: false,
            min_hex_units: None,
            regs: [0; 6],
        };
    }

    pub fn with_min_hex_units(&mut self, min_hex_units: usize) -> &mut Self {
        self.min_hex_units = Some(min_hex_units);
        return self;
    }

    pub fn with_max_color_segments(&mut self, max_color_segments: usize) -> &mut Self {
        self.max_color_segments = Some(max_color_segments);
        return self;
    }

    pub fn as_disabled(&mut self) -> &mut Self {
        self.disabled = true;
        return self;
    }

    pub fn with_initializer(&mut self, func: fn(effect: &mut Effect, context: &Context) -> Context) -> &mut Self {
        self.initializer = Some(func);
        return self;
    }

    fn add_node(&mut self, node: EffectNode) {
        let ptr = alloc();
        unsafe {
            (*ptr) = node;
        }

        if self.root.is_none() {
            self.root = Some(ptr);
        } else {
            let mut tail_ptr = self.root.unwrap();
            while unsafe { tail_ptr.as_mut().unwrap().next.is_some() } {
                tail_ptr = unsafe { (*tail_ptr).next.unwrap() };
            }

            unsafe { (*tail_ptr).next = Some(ptr) };
        }
    }

    pub fn transition_to(&mut self, target: u32, duration: uNano) -> &mut Self {
        self.add_node(EffectNode {
            duration: duration,
            target: target,
            hold: false,
            next: None,
        });

        self.total_time += duration;
        return self;
    }

    pub fn transition_to_and_hold(&mut self, target: u32, duration: uNano) -> &mut Self {
        self.add_node(EffectNode {
            duration: duration,
            target: target,
            hold: true,
            next: None,
        });

        self.total_time += duration;
        return self;
    }


    pub fn build(&mut self) -> Self {
        return self.clone();
    }

    pub fn process(&mut self, ctx: &mut Context, current_time: uNano) -> (uNano, Context) {
        let mut next_context = ctx.clone();

        if !ctx.initialized {
            if self.initializer.is_some() {
                next_context = self.initializer.unwrap()(self, &ctx);
            }
            next_context.initialized = true;
        }
        
        let normalized_time  = (next_context.offset + current_time) % self.total_time;

        if self.root.is_none() {
            return (0, next_context);
        } else {
            let mut ptr = self.root.unwrap();

            let mut start_time = 0;
            let mut elapsed = 0;

            loop {
                let node = unsafe { *ptr };
                let node_duration = match node.hold {
                    false => node.duration,
                    true => node.duration + next_context.offset,
                };

                
                if elapsed + node_duration > normalized_time {
                    break;
                }
                
                elapsed += node.duration;
                start_time = node.target;
                if node.next.is_some() {
                    ptr = node.next.unwrap();
                } else {
                    break;
                }
                
            }

            let root_node = unsafe { *ptr };
            let duration = match root_node.hold {
                false => root_node.duration,
                true => root_node.duration + next_context.offset,
            };
            // Compute the next time
            let target_time = unsafe { (*ptr).target };
            let next_time = interpolate(start_time, target_time, normalized_time - elapsed, duration);
        
            return (next_time as uNano, next_context);
        }
    }
}

#[cfg(test)]
pub mod test_effects {
    use super::*;

    #[test]
    fn test_effects() {
        // let fx = Effect::new(b"Sample")
        //     .randomize(|node_id, total_nodes| {
        //         return rand() % 100;
        //     })
        //     .transition_to_without_delay(100, 500)
        //     .transition_to(0, 500)
        //     .build();


    }
}