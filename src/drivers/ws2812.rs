use teensycore::phys::irq::{disable_interrupts, enable_interrupts};
use teensycore::phys::pins::*;
use teensycore::{wait_ns, wait_exact_ns, MICRO_TO_NANO, clock::uNano};

// 800MHz
const T0_H: uNano = 110; // ns
const T0_L: uNano = 600;
const T1_H: uNano = 600; // ns
const T1_L: uNano = 600;


#[derive(Clone, Copy)]
struct Node {
    pub green: u8,
    pub red: u8,
    pub blue: u8,
}

impl Node {
    pub const fn new(red: u8, green: u8, blue: u8) -> Node {
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
    iteration: usize,
}

impl<const SIZE: usize> WS2812Driver<SIZE> {
    pub const fn new(pin: usize) -> WS2812Driver::<SIZE> {
        return WS2812Driver::<SIZE> {
            nodes: [Node::new(0, 0, 0); SIZE],
            pin: pin,
            iteration: 0,
        }
    }

    pub fn init(&self) {
        // Configure the pin
        pin_mode(self.pin, Mode::Output);
        pin_pad_config(self.pin, PadConfig {
            hysterisis: false,               // HYS
            resistance: PullUpDown::PullDown100k, // PUS
            pull_keep: PullKeep::Pull,            // PUE
            pull_keep_en: false,             // PKE
            open_drain: false,               // ODE
            speed: PinSpeed::Max200MHz,                // SPEED
            drive_strength: DriveStrength::MaxDiv3,  // DSE
            fast_slew_rate: true,           // SRE
        });

        pin_out(self.pin, Power::Low);
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
        wait_exact_ns(T1_H);
        pin_out(self.pin, Power::Low);
        wait_exact_ns(T1_L);
    }
    
    fn off_bit(&self) {        
        pin_out(self.pin, Power::High);
        wait_exact_ns(T0_H);
        pin_out(self.pin, Power::Low);
        wait_exact_ns(T0_L);
    }

    fn rest(&self) {
        pin_out(self.pin, Power::Low);
        wait_ns(3500 * MICRO_TO_NANO);
    }

    pub fn iterate(&mut self) {
        self.iteration += 1;
    }

    pub fn flush(&self) {
        let mut node_index = 0;
        let mut bit_index: i32;
        
        while node_index < SIZE {
            let node = self.nodes[node_index];
            let color: u32 = 
                ((node.green as u32) << 16) |
                ((node.red as u32) << 8) |
                (node.blue as u32); 

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