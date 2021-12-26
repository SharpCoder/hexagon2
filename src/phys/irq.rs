#![allow(dead_code)]

type Ptr = fn();
use core::arch::asm;
use crate::phys::{ 
    addrs,
    assign,
    read_word,
    set_bit,
 };

#[no_mangle]
#[link_section = "vectors"]
static mut VEC_TABLE: [Ptr; 256] = [noop; 256];

/** The qty of registers to skip in order to get to IRQ0 */
pub const IRQ_0_OFFSET: usize = 16;

/** Interrupts */
pub enum Irq {
    GPT1 = 100,
    GPT2 = 101,
    PIT = 122,
}

pub fn enable_interrupts() {
    unsafe {
        asm!("CPSIE i");
    }
}

pub fn disable_interrupts() {
    unsafe {
        asm!("CPSID i");
    }

}

pub fn enable_irq(irq_number: Irq) {
    let num = irq_number as u8;
    let bank = num / 32;
    let bit = num - bank * 32;
    let addr = addrs::NVIC_IRQ_ENABLE_REG + (bank as u32 * 4);
    let original_value = read_word(addr);
    let next_value = set_bit(original_value, bit);
    assign(addr, next_value);
}

pub fn attach_irq(irq_number: Irq, ptr: Ptr) {
    unsafe {
        VEC_TABLE[IRQ_0_OFFSET + irq_number as usize] = ptr;
    }
}


pub fn noop() {
    loop {
        unsafe {
            asm!("nop");
        }
    }
}