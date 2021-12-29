/**
    This file is dedicated to gpio over any discreet pin for the teensy 4.0
    as numbered in the Arduino style (and board diagrams).
*/
use crate::phys::addrs;
use crate::phys::*;
use crate::phys::gpio::*;

pub enum Mode {
    Output,
    Input,
}

pub enum Power {
    High,
    Low,
}

pub enum Alt {
    Alt0 = 0x0,
    Alt1 = 0x1,
    Alt2 = 0x2,
    Alt3 = 0x3,
    Alt4 = 0x4,
    Alt5 = 0x5,
}

pub const PCR_DSE: u32 = 0x40; // Drive Strength Enable
pub const PCR_ODE: u32 = 0x20; // Open Drain Enable
pub const PCR_PFE: u32 = 0x10; // Passive Filter Enable
pub const PCR_SRE: u32 = 0x04; // Slew Rate Enable
pub const PCR_PE: u32 = 0x02; // Pull Enable
pub const PCR_PS: u32 = 0x01; // Pull Select

/** The index is an arduino pin, the output is the teensy 4.0 bit */
const PIN_BITS: [u8; 40] = [
    3, 2, 4, 5, 6, 8, 10, 17, 16, 11, 0,
    2, 1, 3, 18, 19, 23, 22, 17, 16, 26, 27,
    24, 25, 12, 13, 30, 31, 18, 31, 23, 22, 12,
    7, 15, 14, 13, 12, 17, 16,
];

/** The index is an arduino pin, the output is the gpio pin that controls it */
const PIN_TO_GPIO_PIN: [Pin; 40] = [
    Pin::Gpio6, Pin::Gpio6, Pin::Gpio9, Pin::Gpio9, Pin::Gpio9, Pin::Gpio9, Pin::Gpio7, Pin::Gpio7,
    Pin::Gpio7, Pin::Gpio7, Pin::Gpio7, Pin::Gpio7, Pin::Gpio7, Pin::Gpio7, Pin::Gpio6, Pin::Gpio6,
    Pin::Gpio6, Pin::Gpio6, Pin::Gpio6, Pin::Gpio6, Pin::Gpio6, Pin::Gpio6, Pin::Gpio6, Pin::Gpio6,
    Pin::Gpio6, Pin::Gpio6, Pin::Gpio6, Pin::Gpio6, Pin::Gpio8, Pin::Gpio9, Pin::Gpio8, Pin::Gpio8,
    Pin::Gpio7, Pin::Gpio9, Pin::Gpio8, Pin::Gpio8, Pin::Gpio8, Pin::Gpio8, Pin::Gpio8, Pin::Gpio8,
];

/** The index is an arduino pin, the output is the IOMUX register which controls it */
const PIN_MUX: [u32;  40] = [
    addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_AD_B0_03, addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_AD_B0_02,
    addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_EMC_04, addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_EMC_05,
    addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_EMC_06, addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_EMC_08,
    addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_B0_10, addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_B1_01,
    addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_B1_00, addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_B0_11,
    addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_B0_00, addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_B0_02,
    addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_B0_01, addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_B0_03,
    addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_AD_B1_02, addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_AD_B1_03,
    addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_AD_B1_07, addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_AD_B1_06,
    addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_AD_B1_01, addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_AD_B1_00,
    addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_AD_B1_10, addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_AD_B1_11,
    addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_AD_B1_08, addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_AD_B1_09,
    addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_AD_B0_12, addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_AD_B0_13,
    addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_AD_B1_14, addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_AD_B1_15,
    addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_EMC_32, addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_EMC_31,
    addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_EMC_37, addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_EMC_36,
    addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_B0_12, addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_EMC_07,
    addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_SD_B0_03, addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_SD_B0_02,
    addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_SD_B0_01, addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_SD_B0_00,
    addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_SD_B0_05, addrs::IOMUXC_SW_MUX_CTL_PAD_GPIO_SD_B0_04,
];


pub fn pin_mux_config(pin: usize, alt: Alt) {
    let addr = PIN_MUX[pin];
    assign(addr, alt as u32);
}

pub fn pin_pad_config(pin: usize, op: Bitwise, value: u32) {
    // -0x1F0 appears to universally be the difference
    // between the MUX_CTRL_PAD and the PAD_CTRL_PAD
    let addr = PIN_MUX[pin] - 0x1F0;
    let original = read_word(addr);
    match op {
        Bitwise::And => {
            assign(addr, original & value);
        },
        Bitwise::Or => {
            assign(addr, original | value);
        },
        Bitwise::Eq => {
            assign(addr, value);
        }
    }
}


/** This method will mux the pin */
pub fn pin_mode(pin: usize, mode: Mode) {
    gpio_speed(&PIN_TO_GPIO_PIN[pin], MuxSpeed::Fast);
    match mode {
        Mode::Output => {
            gpio_direction(&PIN_TO_GPIO_PIN[pin], Dir::Output);
        },
        Mode::Input => {
            gpio_direction(&PIN_TO_GPIO_PIN[pin], Dir::Input);
        }
    }
}

/** This method will output a high or low signal to the pin */
pub fn pin_out(pin: usize, power: Power) {
    let mask = 0x1 << PIN_BITS[pin];
    match power {
        Power::High => {
            gpio_set(&PIN_TO_GPIO_PIN[pin], mask);
        },
        Power::Low => {
            // assign(reg + 34, 0x1 << PIN_BITS[pin]);
            gpio_clear(&PIN_TO_GPIO_PIN[pin], mask);
        }
    }
}