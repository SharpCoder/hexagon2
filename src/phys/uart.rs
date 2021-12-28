use crate::phys::addrs;
use crate::phys::*;

pub enum ParityType {
    Even,
    Odd,
}

pub enum BitMode {
    NineBits,
    EightBits,
}

#[derive(Clone, Copy)]
pub enum IdleConfiguration {
    Idle1Char = 0x0,
    Idle2Char = 0x1,
    Idle4Char = 0x2,
    Idle8Char = 0x3,
    Idle16Char = 0x4,
    Idle32Char = 0x5,
    Idle64Char = 0x6,
    Idle128Char = 0x7,
}

pub enum Device {
    Uart1,
    Uart2,
    Uart3,
    Uart4,
    Uart5,
    Uart6,
    Uart7,
    Uart8,
}

pub struct UartConfig {
    // R8T9 not supported
    // R9T8 not supported
    // TXDIR not supported currently
    pub invert_transmission_polarity: bool,
    pub overrun_irq_en: bool,
    pub noise_error_irq_en: bool,
    pub framing_error_irq_en: bool,
    pub parity_error_irq_en: bool,
    pub tx_irq_en: bool,
    pub tx_complete_irq_en: bool,
    pub rx_irq_en: bool,
    pub idle_line_irq_en: bool,
    pub tx_en: bool,
    pub rx_en: bool,
    // Receiver wakeup control not supported
    // SBK not currently supported
    pub match1_irq_en: bool,
    pub match2_irq_en: bool,
    // 7-bit mode not supported
    pub idle_config: IdleConfiguration,
    // Loops not supported
    pub doze_en: bool,
    // RSRC not supported
    pub bit_mode: BitMode,
    // Received wakeup not supported
    // Line idle type not supported
    pub parity_en: bool,
    pub parity_type: ParityType,
}

fn set_bit_from_bool(baseline: u32, bit: u8, value: bool) -> u32 {
    if value {
        return set_bit(baseline, bit);
    } else {
        return clear_bit(baseline, bit);
    }
}

fn config_to_u32(config: &UartConfig, baseline: u32) -> u32 {
    let mut result: u32 = baseline;
    
    match config.parity_type {
        ParityType::Even => {
            result = clear_bit(result, 0);
        },
        ParityType::Odd => {
            result = set_bit(result, 0);
        }
    }
    
    result = set_bit_from_bool(result, 1, config.parity_en);

    match config.bit_mode {
        BitMode::NineBits => {
            result = set_bit(result, 4);
        },
        BitMode::EightBits => {
            result = clear_bit(result, 4);
        }
    }

    result = set_bit_from_bool(result, 6, config.doze_en);

    // Clear idle config from original result
    result = result & !(0x7 << 8);
    result = result & (config.idle_config as u32) << 8;

    result = set_bit_from_bool(result, 14, config.match2_irq_en);
    result = set_bit_from_bool(result, 15, config.match1_irq_en);
    result = set_bit_from_bool(result, 18, config.rx_en);
    result = set_bit_from_bool(result, 19, config.tx_en);
    result = set_bit_from_bool(result, 20, config.idle_line_irq_en);
    result = set_bit_from_bool(result, 21, config.rx_irq_en);
    result = set_bit_from_bool(result, 22, config.tx_complete_irq_en);
    result = set_bit_from_bool(result, 23, config.tx_irq_en);
    result = set_bit_from_bool(result, 24, config.parity_error_irq_en);
    result = set_bit_from_bool(result, 25, config.framing_error_irq_en);
    result = set_bit_from_bool(result, 26, config.noise_error_irq_en);
    result = set_bit_from_bool(result, 27, config.overrun_irq_en);
    result = set_bit_from_bool(result, 28, config.invert_transmission_polarity);

    return result;
}

pub fn get_addr(device: &Device) -> u32 {
    return match device {
        Device::Uart1 => addrs::UART1,
        Device::Uart2 => addrs::UART2,
        Device::Uart3 => addrs::UART3,
        Device::Uart4 => addrs::UART4,
        Device::Uart5 => addrs::UART5,
        Device::Uart6 => addrs::UART6,
        Device::Uart7 => addrs::UART7,
        Device::Uart8 => addrs::UART8,
    };
}

// Set the software reset pin on or off
pub fn uart_sw_reset(device: &Device, sw_reset: bool) {
    let value = match sw_reset {
        true => 0x2,
        false => 0x0,
    };

    assign(get_addr(device), value);
}

pub fn uart_configure(device: &Device, configuration: UartConfig) {
    let addr = get_addr(device) + 0x18;
    assign(addr, config_to_u32(&configuration, 0x0));
}

pub fn uart_disable(device: &Device) {
    let addr = get_addr(device) + 0x18;
    let baseline = read_word(addr);
    assign(addr, baseline & !(0x1 << 18) & !(0x1 << 19));
}

pub fn uart_enable(device: &Device) {
    let addr = get_addr(device) + 0x18;
    let baseline = read_word(addr);
    assign(addr, baseline & (0x1 << 18) & (0x1 << 19));
}