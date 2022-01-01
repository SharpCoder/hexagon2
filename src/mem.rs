/*
    Author: Josh Cole
    This library is dedicated to managing memory.
*/
#![allow(dead_code, unused_imports)]

use core::mem::{size_of};
use crate::phys::addrs::OCRAM2;

extern "C" {
     static _heap_end: u32; // Thank you, linker
}

const MEMORY_MAXIMUM: u32 = 0x7D_0000; // 512kb
static mut MEMORY_OFFSET: u32 = 0xD_000;

pub fn alloc(bytes: usize) -> *mut u32 {
    // Check for boundaries and reset if applicable.
    unsafe {
        if MEMORY_OFFSET + bytes as u32 > MEMORY_MAXIMUM {
            MEMORY_OFFSET = 0;
        }

        let ptr = (_heap_end + MEMORY_OFFSET) as *mut u32;
        MEMORY_OFFSET += bytes as u32;
        return ptr;
    }
}

/// This is kernal alloc and it implements what I call a "cyclical mempage strategy".
/// Memory is constantly allocated in RAM and eventually will loop back around
/// if all memory is used up. Clearly, this is a bad idea. Will be improved over time.
pub fn kalloc<T>() -> *mut T {
    let bytes = size_of::<T>();
    return alloc(bytes) as *mut T;
}

/// Free a pointer by updating the pagefile
pub fn free<T>(ptr: *mut T) {
    let bytes = size_of::<T>();
    let zero_ptr = ptr as *mut u32;
    for i in 0 .. bytes / 4 {
        unsafe { 
            *(zero_ptr.offset(i as isize)) = 0;
        }
    }
}