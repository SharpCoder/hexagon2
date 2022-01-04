/*
    Author: Josh Cole
    This library is dedicated to managing memory.
*/
#![allow(dead_code, unused_imports)]
#[cfg(test)]
use std::alloc::{alloc, Layout};
use core::mem::{size_of};

#[cfg(not(test))]
use crate::phys::*;

#[cfg(not(test))]
use crate::phys::addrs::OCRAM2;

const MEMORY_MAXIMUM: u32 = 0x7_FFFF; // 512kb
const MEMORY_BEGIN_OFFSET: u32 = 0x0_FFC; // 4kb buffer (note: it should be word aligned)
static mut MEMORY_OFFSET: u32 = MEMORY_BEGIN_OFFSET;


#[cfg(test)]
pub fn kalloc<T>() -> *mut T {
    return unsafe { alloc(Layout::new::<T>()) as *mut T };
}

#[cfg(test)]
pub fn free<T>(ptr: *mut T) {
    // Do nothing
}

#[cfg(not(test))]
pub fn alloc(bytes: usize) -> *mut u32 {
    // Check for boundaries and reset if applicable.
    unsafe {
        if MEMORY_OFFSET + bytes as u32 > MEMORY_MAXIMUM {
            MEMORY_OFFSET = MEMORY_BEGIN_OFFSET;
            crate::err();
        }

        let ptr = (OCRAM2 + MEMORY_OFFSET) as *mut u32;
        MEMORY_OFFSET += bytes as u32;
        return ptr;
    }
}

/// This is kernal alloc and it implements what I call a "cyclical mempage strategy".
/// Memory is constantly allocated in RAM and eventually will loop back around
/// if all memory is used up. Clearly, this is a bad idea. Will be improved over time.
#[cfg(not(test))]
pub fn kalloc<T>() -> *mut T {
    let bytes = size_of::<T>();
    return alloc(bytes) as *mut T;
}

/// Free a pointer by updating the pagefile
#[cfg(not(test))]
pub fn free<T>(ptr: *mut T) {
    let bytes = size_of::<T>() as u32;
    let zero_ptr = ptr as u32;
    for i in 0u32 .. (bytes / 4) {
        assign(zero_ptr + (i * 4), 0);
    }
}