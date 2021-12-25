#![feature(lang_items)]
#![crate_type = "staticlib"]
#![no_std]


// Import assembly macro
use core::arch::asm;

static IMXRT_IOMUXC: u32 = 0x401F8000;
static IMXRT_IOMUXC_GPR: u32 = 0x400AC000;
static IMXRT_GPIO7: u32 = 0x4200_4000;

#[no_mangle]
pub fn main() {
    unsafe {
        *((IMXRT_IOMUXC_GPR + 0x40) as *mut u32) = 0x0000_0007;
        *((IMXRT_IOMUXC_GPR + 0x6c) as *mut u32) = 0xFFFF_FFFF;
        *((IMXRT_GPIO7 + 0x4) as *mut u32) = 0x1 << 3;
    }

    loop { 
        unsafe {    
            *((IMXRT_GPIO7 + 0x84) as *mut u32) = 0x1 << 3;
            asm!("nop");
        }

    }
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}#[panic_handler]

#[no_mangle]
pub extern fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop { }
}