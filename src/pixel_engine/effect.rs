use teensycore::debug;
use teensycore::mem::*;
use crate::pixel_engine::math::*;
use crate::pixel_engine::context::*;
#[derive(Copy, Clone)]
pub struct EffectNode {
    pub duration: u64,
    pub target: u32,
    hold: bool,
    next: Option<*mut EffectNode>,
}

#[derive(Clone, Copy)]
pub struct Effect {
    name: &'static [u8],
    initializer: Option<fn(context: &Context) -> Context>,
    root: Option<*mut EffectNode>,
    total_time: u64,
}

impl Effect {
    pub fn new(name: &'static [u8]) -> Self {
        return Effect { 
            name: name,
            initializer: None,
            root: None,
            total_time: 0,
        };
    }

    pub fn with_initializer(&mut self, func: fn(context: &Context) -> Context) -> &mut Self {
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

    pub fn transition_to(&mut self, target: u32, duration: u64) -> &mut Self {
        self.add_node(EffectNode {
            duration: duration,
            target: target,
            hold: false,
            next: None,
        });

        self.total_time += duration;
        return self;
    }

    pub fn transition_to_and_hold(&mut self, target: u32, duration: u64) -> &mut Self {
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

    pub fn process(&mut self, ctx: &mut Context, current_time: u64) -> (u64, Context) {
        let mut next_context = ctx.clone();

        if !ctx.initialized {
            if self.initializer.is_some() {
                next_context = self.initializer.unwrap()(&ctx);
            }
            next_context.initialized = true;
        }
        
        let normalized_time = (next_context.offset + current_time) % self.total_time;
        if self.root.is_none() {
            return (0, next_context);
        } else {
            let mut ptr = self.root.unwrap();

            let mut start_time = 0;
            let mut elapsed = 0;

            loop {
                let node = unsafe { (*ptr) };
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
            let mut duration = match root_node.hold {
                false => root_node.duration,
                true => root_node.duration + next_context.offset,
            };
            // Compute the next time
            let target_time = unsafe { (*ptr).target };
            let next_time = interpolate(start_time, target_time, normalized_time - elapsed, duration);
        
            return (next_time as u64, next_context);
        }
        return (0, next_context);
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