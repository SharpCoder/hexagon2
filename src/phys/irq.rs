#![allow(dead_code)]

type Ptr = fn();
use core::arch::asm;
use crate::phys::{ 
    addrs,
    assign,
    read_word,
    set_bit,
 };

const MAX_SUPPORTED_IRQ: usize = 1024;

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
    Uart1 = 20,
    Uart2 = 21,
    Uart3 = 22,
    Uart4 = 23,
    Uart5 = 24,
    Uart6 = 25,
    Uart7 = 26,
    Uart8 = 29,
    Gpt1 = 100,
    Gpt2 = 101,
    PeriodicTimer = 122,
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

// Return the current address stored
// in the NVIC
pub fn irq_addr() -> u32 {
    return read_word(0xe000ed08);
}

// Return the total size of the IVT
pub fn irq_size() -> u32 {
    return core::mem::size_of::<IrqTable>() as u32;
}

// Get the current IVT wherever it may be stored
pub fn get_ivt() -> *mut IrqTable {
    return read_word(0xe000ed08) as *mut IrqTable
}

// Enable a specific interrupt
pub fn irq_enable(irq_number: Irq) {
    let num = irq_number as u32;
    let bank = num / 32;
    let bit = num - bank * 32;
    let addr = addrs::NVIC_IRQ_ENABLE_REG + (bank * 4);
    let original_value = read_word(addr);
    let next_value = set_bit(original_value, bit as u8);
    assign(addr, next_value);
}

// Disable a specific interrupt
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


/**
This method exists to copy the "shadow NVIC" into
the real NVIC.

Why?

The actual address stored in the NVIC changes sometimes.
I don't know why. But it seems like the data gets copied
to a new location randomly. I have my theories, and it
seems to only happen after the stack pointer
is moved around. So anyway, this is just the nuclear
approach to really fkn thoroughly making sure
the NVIC has the value I think it has.

Spent like 20 hours debugging this. I am so done
with magic memory locations changing around.
*/
fn update_ivt() {
    let ivt = get_ivt();
    unsafe {
        (*ivt).init_sp =  VECTORS.init_sp;
        (*ivt).reset_handler = VECTORS.reset_handler;
        (*ivt).nmi_handler = VECTORS.nmi_handler;
        (*ivt).hardfault_handler = VECTORS.hardfault_handler;
        (*ivt).mpufault_handler = VECTORS.mpufault_handler;
        (*ivt).busfault_handler = VECTORS.busfault_handler;
        (*ivt).usagefault_handler = VECTORS.usagefault_handler;
        (*ivt).rsv0 = VECTORS.rsv0;
        (*ivt).rsv1 = VECTORS.rsv1;
        (*ivt).rsv2 = VECTORS.rsv2;
        (*ivt).rsv3 = VECTORS.rsv3;
        (*ivt).svc_handler = VECTORS.svc_handler;
        (*ivt).rsv4 = VECTORS.rsv4;
        (*ivt).rsv5 = VECTORS.rsv5;
        (*ivt).svc_handler = VECTORS.svc_handler;
        (*ivt).pendsv_handler = VECTORS.pendsv_handler;
        (*ivt).systick_handler = VECTORS.systick_handler;
        let mut i = 0;
        while i < MAX_SUPPORTED_IRQ {
            (*ivt).interrupts[i] = VECTORS.interrupts[i];
            i += 1;
        }
    }
}

// Internal method for assigning a specific irq
// at a specific index to the IVT.
fn put_irq(irq_number: usize, ptr: Ptr) {
    unsafe {
        // Update shadow copy
        VECTORS.interrupts[irq_number] = ptr;
        // Copy shadow to actual NVIC
        update_ivt();
    }
}

// DO NOT USE!!!
// Unless you know what you are doing
pub fn fill_irq(ptr: Ptr) {
    let mut index = 0;
    while index < MAX_SUPPORTED_IRQ {
        put_irq(index, ptr);
        index += 1;
    }
}

// Public method for attaching an interrupt to an
// enum-gated IRQ source.
pub fn irq_attach(irq_number: Irq, ptr: Ptr) {
    put_irq(irq_number as usize, ptr);
}

// Some kind of hard-fault, typically
// this is a catastrophic function that hangs
// the program.
pub fn fault_handler() {
    crate::err();
}

// An un-implemented interrupt
pub fn noop() {
    unsafe {
        asm!("nop");
    }
}