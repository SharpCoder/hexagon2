/*
Gating is what allows tasks in the RTOS to execute 
quickly. Think of it like a thread mixed with a promise. 
The gating mechanism defines a flow governed by a 
condition statemenet and then a function delegate. 
If the condition is not met, the gate will yield 
until a later point.
*/
use crate::clock::*;

type CondFn = fn(&mut Gate) -> bool;
type Fn = fn();

const MAX_SUPPORTED_FUNCTIONS: usize = 128;

#[derive(Copy, Clone)]
pub struct Gate {
    conditions: [CondFn; MAX_SUPPORTED_FUNCTIONS],
    functions: [Fn; MAX_SUPPORTED_FUNCTIONS],
    durations: [u64; MAX_SUPPORTED_FUNCTIONS],
    target_times: [u64; MAX_SUPPORTED_FUNCTIONS],
    current_index: usize,
    tail: usize,
}

impl Gate {
    pub fn new() -> Gate {
        return Gate {
            conditions: [base_cond_fn; MAX_SUPPORTED_FUNCTIONS],
            functions: [base_fn; MAX_SUPPORTED_FUNCTIONS],
            durations: [0u64; MAX_SUPPORTED_FUNCTIONS],
            target_times: [0u64; MAX_SUPPORTED_FUNCTIONS],
            current_index: 0usize,
            tail: 0usize,
        }
    }

    pub fn when(&mut self, cond: CondFn, then: Fn) -> &mut Self {
        self.conditions[self.tail] = cond;
        self.functions[self.tail] = then;
        self.tail += 1;
        return self;
    }

    pub fn when_nano(&mut self, duration_nanos: u64, then: Fn) -> &mut Self {
        self.conditions[self.tail] = |this: &mut Gate| {
            return nanos() > this.target_times[this.current_index];
        };
        self.functions[self.tail] = then;
        self.durations[self.tail] = duration_nanos;
        self.tail += 1;
        return self;
    }

    pub fn compile(&mut self) -> Gate {
        return *self;
    }

    // This method will evaluate the current
    // gate condition and, if true, execute
    // the underlying block.
    pub fn process(&mut self) {
        if self.conditions[self.current_index](self) {
            self.functions[self.current_index]();
            self.current_index += 1;

            if self.current_index == self.tail {
                self.current_index = 0;
            } 

            self.target_times[self.current_index] = nanos() + self.durations[self.current_index];
        }

    }
}

fn base_cond_fn(_this: &mut Gate) -> bool {
    return true;
}

fn base_fn() {
    return;
}