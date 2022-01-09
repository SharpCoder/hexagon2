#![allow(dead_code, unused_imports)]
#[cfg(test)]
use std::alloc::{alloc, Layout};
use core::mem::{size_of};

#[cfg(not(test))]
use crate::*;
#[cfg(not(test))]
use crate::debug::*;
#[cfg(not(test))]
use crate::phys::*;
#[cfg(not(test))]
use crate::phys::addrs::OCRAM2;

const MEMORY_MAXIMUM: u32 = 0x7_FFFF; // 512kb
const MEMORY_BEGIN_OFFSET: u32 = 0x0_0FFC; // 4kb buffer (note: it should be word aligned)
static mut MEMORY_OFFSET: u32 = MEMORY_BEGIN_OFFSET;
static mut MEMORY_PAGES: Option<*mut Mempage> = None;
pub static mut MEMORY_OVERFLOW: bool = false;

/// A page of memory
#[repr(C)]
pub struct Mempage {
    pub size: usize,
    pub used: bool,
    pub next: Option<*mut Mempage>,
    pub ptr: *mut u32,
}

#[cfg(not(test))]
impl Mempage {
    pub const fn new(size: usize, ptr: *mut u32) -> Self {
        return Mempage {
            size: size,
            used: true,
            ptr: ptr,
            next: None,
        };
    }

    pub fn reclaim(bytes: usize) -> *mut u32 {
        // Iterate through mempage
        unsafe {
            let mut ptr = MEMORY_PAGES;
            while ptr.is_some() {
                let node = ptr.unwrap();
                if (*node).size >= bytes && (*node).used == false {
                    (*node).used = true;
                    return node as *mut u32;
                }
                ptr = (*node).next;
                
            }
        }

        loop {
            crate::err(crate::PanicType::Memfault);
        }
    }

    /// Free the page containing this ptr
    pub fn free(ptr: u32) {
        let bytes = size_of::<Mempage>() as u32;
        // We know the Mempage header is
        // right above the pointer. So we can use
        // that knowledge to go straight there.
        let addr = (ptr - bytes) as *mut Mempage;
        unsafe {
            (*addr).used = false;
        }
    }

    pub fn add_page<T>(bytes: usize) -> *mut T {
        let page_bytes = size_of::<Mempage>();
        let mut total_bytes = page_bytes + bytes;

        // Word align
        while total_bytes % 4 != 0 {
            total_bytes += 1;
        }

        let next_page = alloc(total_bytes) as *mut Mempage;
        let item_ptr = ((next_page as u32) + page_bytes as u32) as *mut T; 

        if unsafe { MEMORY_OVERFLOW } {
            // Don't allocate a new page
            return item_ptr;
        }

        unsafe {
            (*next_page) = Mempage {
                size: total_bytes,
                ptr: item_ptr as *mut u32,
                used: true,
                next: None,
            };
            
            match MEMORY_PAGES {
                None => {
                    MEMORY_PAGES = Some(next_page);
                },
                Some(head) => {
                    (*next_page).next = Some(head);
                    MEMORY_PAGES = Some(next_page);
                }
            }
        }

        return item_ptr;

    }
}

/// zero out every piece of memory.
/// if we encounter a bad sector,
/// the device will throw an oob irq
/// and enter error mode.
#[cfg(not(test))]
pub fn memtest() {
    for addr in MEMORY_BEGIN_OFFSET .. MEMORY_MAXIMUM / 4 {
        unsafe {
            let ptr = (OCRAM2 + addr * 4) as *mut u32;
            *ptr = 0;
        }
    }
}

#[cfg(not(test))]
pub fn alloc(bytes: usize) -> *mut u32 {
    // Check for boundaries and reset if applicable.
    unsafe {
        if MEMORY_OFFSET + bytes as u32 >= MEMORY_MAXIMUM {
            MEMORY_OVERFLOW = true;
            return Mempage::reclaim(bytes);
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
    return Mempage::add_page(bytes);
}

/// Free a pointer by updating the pagefile
#[cfg(not(test))]
pub fn free<T>(ptr: *mut T) {
   let zero_ptr = ptr as u32;
    Mempage::free(zero_ptr);
}

#[cfg(test)]
pub fn kalloc<T>() -> *mut T {
    return unsafe { alloc(Layout::new::<T>()) as *mut T };
}

#[cfg(test)]
pub fn free<T>(_ptr: *mut T) {
    // Do nothing
}
