#![allow(dead_code)]
pub const NVIC_IRQ_ENABLE_REG: u32 = 0xE000E100;
pub const NVIC_IRQ_CLEAR_REG: u32 = 0xE000E180;
pub const NVIC_IRQ_CLEAR_PENDING_REG: u32 = 0xE000E280;
/** UART */
pub const UART1: u32 = 0x4018_4000;
pub const UART2: u32 = 0x4018_8000;
pub const UART3: u32 = 0x4018_C000;
pub const UART4: u32 = 0x4019_0000;
pub const UART5: u32 = 0x4019_4000;
pub const UART6: u32 = 0x4019_8000;
pub const UART7: u32 = 0x4019_C000;
pub const UART8: u32 = 0x401A_0000;
/** Fast Ram (512kb) */
pub const OCRAM: u32 = 0x2028_0000;
/** Slow Ram (512kb) */
pub const OCRAM2: u32 = 0x2020_0000;
/** GPIO Registers */
pub const GPIO1: u32 = 0x401B_8000;
pub const GPIO2: u32 = 0x401B_C000;
pub const GPIO3: u32 = 0x401C_0000;
pub const GPIO4: u32 = 0x401C_4000;
pub const GPIO5: u32 = 0x400C_0000;
pub const GPIO6: u32 = 0x4200_0000;
pub const GPIO7: u32 = 0x4200_4000;
pub const GPIO8: u32 = 0x4200_8000;
pub const GPIO9: u32 = 0x4200_C000;
/** General Purpose Timers */
pub const GPT1: u32 = 0x401E_C000;
pub const GPT2: u32 = 0x401F_0000;
/** Periodic Timers */
pub const PIT: u32 = 0x4008_4000;
/** GPIO General Purpose Registers */
pub const IOMUXC_GPR_GPR26: u32 = 0x400A_C068; // GPIO1 and GPIO6 mux settings
pub const IOMUXC_GPR_GPR27: u32 = 0x400A_C06C; // GPIO2 and GPIO7 mux settings
pub const IOMUXC_GPR_GPR28: u32 = 0x400A_C070; // GPIO3 and GPIO8 mux settings
pub const IOMUXC_GPR_GPR29: u32 = 0x400A_C074; // GPIO4 and GPIO9 mux settings
/** GPIO Mux Pads */
pub const SW_MUX_CTL_PAD_GPIO_B0_14: u32 = 0x401F_8174;
pub const IOMUXC_SW_MUX_CTL_PAD_GPIO_B0_03: u32 = 0x401F_8148; // Gpio2 and Gpio7 - Pin 13
pub const IOMUXC_SW_MUX_CTL_PAD_GPIO_AD_B0_12: u32 = 0x401F_80EC; // UART1
pub const IOMUXC_SW_PAD_CTL_PAD_GPIO_AD_B0_12: u32 = 0x401F_82DC;
pub const IOMUXC_SW_MUX_CTL_PAD_GPIO_AD_B0_14: u32 = 0x401F_80F4;
/** Misc */
pub const CCM_CSCMR1: u32 = 0x400F_C01C; // CCM Serial Clock Multiplexer Register 1
pub const CCM_CCGR1: u32 = 0x400F_C06C; // Clock Gating Register 1