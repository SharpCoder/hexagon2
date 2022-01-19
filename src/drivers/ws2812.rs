use teensycore::phys::irq::{disable_interrupts, enable_interrupts};
use teensycore::phys::pins::*;
use teensycore::{wait_ns, MICRO_TO_NANO};

// Who tf knows. Magic number.
const MODIFIER: u64 = 2;

const H_H: u64 = 1360 / MODIFIER;
const H_L: u64 = 350 / MODIFIER;
const L_H: u64 = 350 / MODIFIER;
const L_L: u64 = 1360 / MODIFIER;


#[derive(Clone, Copy)]
struct Node {
    pub green: u8,
    pub red: u8,
    pub blue: u8,
}

impl Node {
    pub fn new(red: u8, green: u8, blue: u8) -> Node {
        return Node {
            red: red,
            green: green,
            blue: blue,
        };
    }
}

#[derive(Copy, Clone)]
pub struct WS2812Driver<const SIZE: usize> {
    nodes: [Node; SIZE],
    pin: usize,
}

impl<const SIZE: usize> WS2812Driver<SIZE> {
    pub fn new(pin: usize) -> WS2812Driver::<SIZE> {
        // Configure the pin
        pin_mode(pin, Mode::Output);
        pin_pad_config(pin, PadConfig {
            hysterisis: false,               // HYS
            resistance: PullUpDown::PullDown100k, // PUS
            pull_keep: PullKeep::Pull,            // PUE
            pull_keep_en: false,             // PKE
            open_drain: false,               // ODE
            speed: PinSpeed::Fast150MHz,                // SPEED
            drive_strength: DriveStrength::Max,  // DSE
            fast_slew_rate: true,           // SRE
        });

        pin_out(pin, Power::Low);
        
        return WS2812Driver::<SIZE> {
            nodes: [Node::new(0, 0, 0); SIZE],
            pin: pin,
        }
    }

    pub fn set_color(&mut self, index: usize, rgb: u32) {
        // Don't process requests out of bounds
        if index >= SIZE {
            return;
        }

        self.nodes[index].red = ((rgb & 0xFF0000) >> 16) as u8;
        self.nodes[index].green = ((rgb & 0x00FF00) >> 8) as u8;
        self.nodes[index].blue = ((rgb & 0x0000FF) >> 0) as u8;
    }

    fn on_bit(&self) {
        pin_out(self.pin, Power::High);
        wait_ns(H_H);
        pin_out(self.pin, Power::Low);
        wait_ns(H_L);
    }
    
    fn off_bit(&self) {
        pin_out(self.pin, Power::High);
        wait_ns(L_H);
        pin_out(self.pin, Power::Low);
        wait_ns(L_L);
    }

    fn rest(&self) {
        pin_out(self.pin, Power::Low);
        wait_ns(50 * MICRO_TO_NANO);
    }

    pub fn flush(&self) {
        let mut node_index = 0;
        let mut bit_index: i32;

        while node_index < SIZE {
            let node = self.nodes[node_index];
            let color: u32 = rgb_to_hex(node.red / 5, node.green / 5, node.blue / 5);

            disable_interrupts();
            // Now we need to process each bit
            bit_index = 23;
            while bit_index >= 0 {
                let bit = color & (0x1 << bit_index);
                if bit > 0 {
                    self.on_bit();
                } else {
                    self.off_bit();
                }
                bit_index -= 1;
            }

            enable_interrupts();
            node_index += 1;
        }

        self.rest();
        
    }
}

pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> u32 {
    return ((r as u32) << 16) |
        ((g as u32) << 8) |
        (b as u32); 
}

pub fn wheel(wheel_pos: u8) -> u32 {
    let mut pos = wheel_pos;
    if pos < 85 {
        return rgb_to_hex(255 - pos * 3, 0, pos * 3);
    } else if pos < 170 {
        pos -= 85;
        return rgb_to_hex(0, pos * 3, 255 - pos * 3);
    } else {
        pos -= 170;
        return rgb_to_hex(pos * 3, 255 - pos * 3, 0);
    }
}
