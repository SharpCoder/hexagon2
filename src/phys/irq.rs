#![allow(dead_code)]

type Ptr = fn();
use core::arch::asm;
use core::arch::global_asm;
use crate::phys::{ 
    addrs,
    assign,
    read_word,
    set_bit,
 };


extern "C" {
    fn set_nvic(addr: *const IrqTable);
}

const MAX_SUPPORTED_IRQ: usize = 256;

#[repr(C)]
pub struct IrqTable {
    pub init_sp: u32, // This gets set magically in c
    pub reset_handler: Ptr,
    pub nmi_handler: Ptr,
    pub hardfault_handler: Ptr,
    pub mpufault_handler: Ptr,
    pub busfault_handler: Ptr,
    pub usagefault_handler: Ptr,
    pub rsv0: u32,
    pub rsv1: u32,
    pub rsv2: u32,
    pub rsv3: u32,
    pub svc_handler: Ptr,
    pub rsv4: u32,
    pub rsv5: u32,
    pub pendsv_handler: Ptr,
    pub systick_handler: Ptr,
    pub interrupts: [Ptr; MAX_SUPPORTED_IRQ],
}

pub static mut VECTORS: IrqTable = IrqTable {
    init_sp: 0x00, // This gets set magically in c
    reset_handler: noop,
    nmi_handler: fault_handler, 
    hardfault_handler: fault_handler,
    mpufault_handler: fault_handler, 
    busfault_handler: fault_handler,
    usagefault_handler: fault_handler,
    rsv0: 0x0,
    rsv1: 0x0,
    rsv2: 0x0,
    rsv3: 0x0,
    svc_handler: noop,
    rsv4: 0x0,
    rsv5: 0x0,
    pendsv_handler: crate::err,
    systick_handler: noop,    
    interrupts: [noop; MAX_SUPPORTED_IRQ],
};

/** Interrupts */
#[derive(Copy, Clone)]
pub enum Irq {
    UART1 = 20,
    UART2 = 21,
    UART3 = 22,
    UART4 = 23,
    UART5 = 24,
    UART6 = 25,
    UART7 = 26,
    UART8 = 29,
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

#[no_mangle]
pub fn irq_init() {
    unsafe {
        set_nvic(&VECTORS);
    }

    // Disable all interrupts
    assign(addrs::NVIC_IRQ_CLEAR_REG + 0x0, 0xFFFF_FFFF);
    assign(addrs::NVIC_IRQ_CLEAR_REG + 0x4, 0xFFFF_FFFF);
    assign(addrs::NVIC_IRQ_CLEAR_REG + 0x8, 0xFFFF_FFFF);
    assign(addrs::NVIC_IRQ_CLEAR_REG + 0xC, 0xFFFF_FFFF);
    // Reset all interrupt vector handlers
    fill_irq(noop);
    // Reset the interrupt pointers
    irq_clear_pending();
}

pub fn irq_enable(irq_number: Irq) {
    let num = irq_number as u32;
    let bank = num / 32;
    let bit = num - bank * 32;
    let addr = addrs::NVIC_IRQ_ENABLE_REG + (bank * 4);
    let original_value = read_word(addr);
    let next_value = set_bit(original_value, bit as u8);
    assign(addr, next_value);
}

pub fn irq_disable(irq_number: Irq) {
    let num = irq_number as u32;
    let bank = num / 32;
    let bit = num - bank * 32;
    let addr = addrs::NVIC_IRQ_CLEAR_REG + (bank * 4);
    let original_value = read_word(addr);
    let next_value = set_bit(original_value, bit as u8);
    assign(addr, next_value);
}

pub fn irq_clear_pending() {
    assign(addrs::NVIC_IRQ_CLEAR_PENDING_REG + 0x0, 0x0);
    assign(addrs::NVIC_IRQ_CLEAR_PENDING_REG + 0x4, 0x0);
    assign(addrs::NVIC_IRQ_CLEAR_PENDING_REG + 0x8, 0x0);
    assign(addrs::NVIC_IRQ_CLEAR_PENDING_REG + 0xC, 0x0);
}

// DO NOT USE!!!
// Unless you know what you are doing
pub fn fill_irq(ptr: Ptr) {
    unsafe {
        let mut index = 0;
        while index < VECTORS.interrupts.len() {
            VECTORS.interrupts[index] = ptr;
            index += 1;
        }
    }
}


pub fn put_irq(irq_number: usize, ptr: Ptr) {
    unsafe {
        VECTORS.interrupts[irq_number] = ptr;
    }
}


pub fn attach_irq(irq_number: Irq, ptr: Ptr) {
    unsafe {
        VECTORS.interrupts[irq_number as usize] = ptr;
        asm!("nop");
    }
}

#[no_mangle]
pub fn fault_handler() {
    crate::err();
}

#[no_mangle]
pub fn noop() {
    unsafe {
        asm!("nop");
    }
}

global_asm!("
    ptr_fn_to_addr_byte:
        add r0, sp, #4
        mov pc, lr

    set_nvic:
        ldr	r3, [pc, #4]
        nop
        str	r0, [r3, #0]
        mov pc, lr
        .word	0xE000ED08
");