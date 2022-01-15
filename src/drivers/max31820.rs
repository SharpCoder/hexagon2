use teensycore::*;
use teensycore::clock::*;
use teensycore::debug::*;
use teensycore::phys::pins::*;

pub struct Max31820Driver {
    pin: usize,
}

impl Max31820Driver {
    pub fn new(data_pin: usize) -> Self {
        pin_mux_config(data_pin, Alt::Alt5);
        return Max31820Driver {
            pin: data_pin,
        };
    }

    pub fn read_temperature(&self) -> Option<u16> {
        // Handshake proceedure
        if self.initialize() {
            self.send_command(0x44); // Convert T command
            let byte_lsb = self.read_byte() as u16;
            let byte_msb = self.read_byte() as u16;
            let result: u16 = (byte_msb << 8) | byte_lsb;
            return Some(result);
        } else {
            debug_str(b"failed to initialize handshake with max31820");
        }

        return None;
    }

    fn as_input(&self) {
        pin_mode(self.pin, Mode::Input);
        pin_pad_config(self.pin, PadConfig { 
            hysterisis: true, 
            resistance: PullUpDown::PullUp47k, 
            pull_keep: PullKeep::Pull, 
            pull_keep_en: true, 
            open_drain: false, 
            speed: PinSpeed::Medium100MHz, 
            drive_strength: DriveStrength::Max, 
            fast_slew_rate: true 
        });
    }

    fn as_output(&self) {
        pin_mode(self.pin, Mode::Output);
        // pin_pad_config(self.pin, PadConfig { 
        //     hysterisis: false, 
        //     resistance: PullUpDown::PullUp47k, 
        //     pull_keep: PullKeep::Pull, 
        //     pull_keep_en: false, 
        //     open_drain: false, 
        //     speed: PinSpeed::Max200MHz, 
        //     drive_strength: DriveStrength::Max, 
        //     fast_slew_rate: false 
        // });
    }

    fn pull_low(&self) {
        self.as_output();
        pin_out(self.pin, Power::Low);
    }


    pub fn initialize(&self) -> bool {
        self.reset();
        if self.detect_pulse() {
            debug_str(b"pulse detected");
            return true;
        } else {
            debug_str(b"could not connect to max31820");
            return false;
        }

        // loop {
        //     pin_out(13, Power::High);
        //     self.as_input();
        //     wait_ns(S_TO_NANO * 2);


        //     pin_out(13, Power::Low);
        //     self.as_output();
        //     pin_out(self.pin, Power::Low);
        //     wait_ns(S_TO_NANO * 2);
        // }
        
    }

    fn reset(&self) {
        self.pull_low();
        wait_ns(MICRO_TO_NANO *  480);
        self.as_input();
    }

    fn detect_pulse(&self) -> bool {
        self.as_input();

        let mut detected = false;
        let mut z= 0;
        let mut h = 0;

        let target = nanos() + MICRO_TO_NANO * 2400;
        while nanos() < target {
            let reading = pin_read(self.pin);
            if pin_read(self.pin) > 0 {
                detected = true;
                h+=1;
            } else {
                z+= 1;
            }
        }

        debug_u32(h, b"high pulses");
        debug_u32(z, b"low pulses");

        return detected;
    }

    fn send_command(&self, command: u8) {
        let mut bit_index: usize = 0;
        while bit_index < 8 {
            let bit = command & (0x1 << bit_index);
            if bit > 0 {
                self.write_1();
            } else {
                self.write_0();
            }
        }
    }

    fn write_1(&self) {
        self.as_output();
        pin_out(self.pin, Power::Low);
        wait_ns(MICRO_TO_NANO * 10);
        
        // Release
        self.as_input();
        wait_ns(MICRO_TO_NANO * 50);
    } 

    fn write_0(&self) {
        self.pull_low();
        wait_ns(MICRO_TO_NANO * 65);

        // Release
        self.as_input();
    }

    fn read_bit(&self) -> u8 {
        // Initiate a read slot
        self.pull_low();
        wait_ns(MICRO_TO_NANO * 1);

        self.as_input();
        // Wait a few microseconds
        wait_ns(MICRO_TO_NANO * 5);
        // Sample for 54 microseconds
        let target = nanos() + MICRO_TO_NANO *  54;
        let mut result = false;
        
        let mut high = 0;
        let mut low = 0;

        while nanos() < target {
            // Sample
            let reading = pin_read(self.pin);
            if reading > 0 {
                high += 1;
            } else {
                low += 1;
            }
        } 

        // Determine the result based on which bucket
        // got more hits.
        if high > low {
            return 1;
        } else {
            return 0;
        }
    }

    fn read_byte(&self) -> u8 {
        let mut result = 0;
        for bit in 0 .. 8 {
            result |= (self.read_bit() << bit);
        }
        return result;
    }
}