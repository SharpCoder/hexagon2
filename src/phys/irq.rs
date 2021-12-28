#![allow(dead_code)]

type Ptr = fn();
use core::arch::asm;
use crate::phys::{ 
    addrs,
    assign,
    read_word,
    set_bit,
    clear_bit,
 };

#[no_mangle]
#[link_section = "vectors"]
static mut VEC_TABLE: [Ptr; 180] = [noop; 180];

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

pub fn irq_init() {
    // Disable all interrupts
    assign(addrs::NVIC_IRQ_CLEAR_REG + 0x0, 0xFFFF_FFFF);
    assign(addrs::NVIC_IRQ_CLEAR_REG + 0x4, 0xFFFF_FFFF);
    assign(addrs::NVIC_IRQ_CLEAR_REG + 0x8, 0xFFFF_FFFF);
    assign(addrs::NVIC_IRQ_CLEAR_REG + 0xC, 0xFFFF_FFFF);
    // Reset all interrupt vector handlers
    let mut handler_idx = 0;
    while handler_idx < (180 - IRQ_0_OFFSET) {
        unsafe {
            VEC_TABLE[IRQ_0_OFFSET + handler_idx] = noop;
        }
        handler_idx += 1;
    }
    // Reset the interrupt pointers
    irq_clear_pending();
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

pub fn disable_irq(irq_number: Irq) {
    let num = irq_number as u8;
    let bank = num / 32;
    let bit = (num - bank * 32);
    let addr = addrs::NVIC_IRQ_CLEAR_REG + (bank as u32 * 4);
    let original_value = read_word(addr);
    let next_value = set_bit(original_value, bit);
    assign(addr, next_value);
}

pub fn irq_clear_pending() {
    assign(addrs::NVIC_IRQ_CLEAR_PENDING_REG + 0x0, 0x0);
    assign(addrs::NVIC_IRQ_CLEAR_PENDING_REG + 0x4, 0x0);
    assign(addrs::NVIC_IRQ_CLEAR_PENDING_REG + 0x8, 0x0);
    assign(addrs::NVIC_IRQ_CLEAR_PENDING_REG + 0xC, 0x0);
}

// DO NOT USE!!!
pub fn fill_irq(ptr: Ptr) {
    unsafe {
        let mut index = 0;
        while index < (180 - IRQ_0_OFFSET) {
            VEC_TABLE[IRQ_0_OFFSET + index] = ptr;
            index += 1;
        }
    }
}

pub fn attach_irq(irq_number: Irq, ptr: Ptr) {
    unsafe {
        let irq = irq_number as usize;
        VEC_TABLE[IRQ_0_OFFSET + irq as usize] = ptr;
        VEC_TABLE[IRQ_0_OFFSET - 4 + irq as usize] = ptr;

        // let mut r: usize = 0;
        // while r < 10 {
        //     VEC_TABLE[IRQ_0_OFFSET - r + irq as usize] = ptr;
        //     VEC_TABLE[IRQ_0_OFFSET + r + irq as usize] = ptr;
        //     r += 1;
        // }
    }
}


pub fn noop() {
    unsafe {
        asm!("nop");
    }
}