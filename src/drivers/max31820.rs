use core::arch::asm;
use teensycore::*;
use teensycore::clock::*;
use teensycore::debug::*;
use teensycore::phys::irq::disable_interrupts;
use teensycore::phys::irq::enable_interrupts;
use teensycore::phys::pins::*;
use teensycore::math::*;
use teensycore::serio::*;

const fn micros(time: u64) -> u64 {
    return MICRO_TO_NANO * time;
}

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

    pub fn test(&self) {
        if self.initialize() {
            self.send_command(0x33);

            for id in 0 .. 64 {
                disable_interrupts();
                let bit = self.read_bit();
                enable_interrupts();

                serial_write_str(SerioDevice::Debug, &itoa(id));
                serial_write(SerioDevice::Debug, b" bit, val=");
                debug_u64(bit as u64, b"bit");
            }
        }
    }

    fn log(&self, msg: &'static [u8]) {
        debug_str(msg);
    }

    fn cmd_convert_t(&self) {
        self.log(b"cmd_convert_t");
        self.send_command(0x44); // Convert T command
        // Read until receiving a 1
        // TODO: Add an oh-shit detector
        loop {
            let bit = self.read_bit();
            // Temperature conversino is done
            if bit > 0 {
                break;
            }
        }
    }

    fn cmd_read_scratchpad(&self) -> Option<u16> {
        self.log(b"cmd_read_scratchpad");
        self.send_command(0xBE);

        // Read bytes
        let byte_lsb = self.read_byte() as u16;
        let byte_msb = self.read_byte() as u16;
        let result: u16 = (byte_msb << 8) | byte_lsb;
        return Some(result);
    }

    fn cmd_skip_rom(&self) -> Option<bool> {
        self.log(b"cmd_skip_rom");
        if self.initialize() {
            self.send_command(0xCC);
            return Some(true);
        }

        return None;
    }

    fn cmd_read_rom(&self) -> Option<u64> {
        self.log(b"cmd_read_rom");
        if self.initialize() {
            self.send_command(0x33);
            
            // Rest period
            wait_ns(micros(1));

            // Read 64 bits     
            let mut family_code = 0;
            for bit in 0 .. 8 {
                family_code |= (self.read_bit() as u64) << bit;
            }

            debug_u64(family_code, b"family code");

            let mut rom_code = 0;       
            for bit in 0 .. 48 {
                rom_code |= (self.read_bit() as u64) << bit;
            }

            debug_u64(rom_code, b"rom code");

            let mut crc = 0u64;
            for bit in 0 .. 8 {
                crc |= (self.read_bit() as u64) << bit;
            }

            debug_u64(crc, b"crc");
            return Some(rom_code);
        }

        return None;
    }

    fn cmd_match_rom(&self, rom: u64) {
        self.log(b"cmd_match_rom");
        // Tell the bus we're about to address a specific node
        self.send_command(0x55);
        let mut bit_index: usize = 0;
        while bit_index < 64 {
            let bit = rom & (0x1 << bit_index);
            if bit > 0 {
                self.write_1();
            } else {
                self.write_0();
            }
            bit_index += 1;
        }
    }

    pub fn read_rom(&self) -> Option<u64> {
        self.log(b"** read rom **");
        return self.cmd_read_rom();
    }

    pub fn read_temperature(&self) -> Option<u16> {
        self.log(b"read_temperature");
        // Get ROM
        self.cmd_skip_rom();
        self.cmd_convert_t();
        self.cmd_skip_rom();
        return self.cmd_read_scratchpad();
    }

    fn as_input(&self) {
        pin_mode(self.pin, Mode::Input);
        pin_pad_config(self.pin, PadConfig { 
            hysterisis: false, 
            resistance: PullUpDown::PullDown100k, 
            pull_keep: PullKeep::Pull, 
            pull_keep_en: false, 
            open_drain: true, 
            speed: PinSpeed::Max200MHz, 
            drive_strength: DriveStrength::Max, 
            fast_slew_rate: false 
        });
    }

    fn as_output(&self) {
        pin_mode(self.pin, Mode::Output);
    }

    fn pull_low(&self) {
        self.as_output();
        pin_out(self.pin, Power::Low);
    }


    fn initialize(&self) -> bool {
        for _ in 0 .. 125 {
            if self.reset() {
                debug_str(b"pulse identified");
                return true;
            }
        }

        debug_str(b"could not connect to max31820");
        return false;
    }

    fn reset(&self) -> bool{
        // Write low
        self.pull_low();
        wait_ns(micros(500));

        // Allow float
        self.as_input();
        wait_ns(micros(70));

        // Wait a while then sample
        let target = nanos() + micros(240);
        let mut result = 1;
        while nanos() < target {

            if pin_read(self.pin) == 0 {
                result = 0;
            }

        }
        
        // Wait 410 micros
        wait_ns(micros(500));

        // If result is 0, that's an alive pulse.
        return result == 0;
    }

    fn send_command(&self, command: u8) {
        for bit in 0 .. 8 {
            let signal = command & (0x1 << bit);
            if signal > 0 {
                self.write_1();
            } else {
                self.write_0();
            }
        }

        self.as_input();
    }

    fn write_1(&self) {
        self.pull_low();
        wait_ns(micros(10));
        // Release
        pin_out(self.pin, Power::High);
        wait_ns(micros(55));
    } 

    fn write_0(&self) {
        self.pull_low();
        wait_ns(micros( 65));

        // Release        // Wait for the recovery time.
        pin_out(self.pin, Power::High);
        wait_ns(micros(5));
    }

    fn read_bit(&self) -> u8 {
        // Initiate a read slot
        self.pull_low();
        wait_ns( micros(3));

        self.as_input();
        wait_ns(micros(12));

        let mut sig_high = 0;
        let mut sig_low = 0;
        let duration = nanos() + micros(1);

        loop {
            let signal = pin_read(self.pin);
            if signal == 0 {
                sig_low += 1;
            } else {
                sig_high += 1;
            }

            if nanos() > duration {
                break;
            }
        }
        
        // Wait the remainder of the time slot
        wait_ns(micros(53));

        // Determine the result based on which bucket
        // got more hits.
        if sig_high > sig_low {
            return 1;
        } else {
            return 0;
        }
    } 

    fn read_byte(&self) -> u8 {
        let mut result = 0;
        for bit in 0 .. 7 {
            result |= self.read_bit() << bit;
        }
        return result;
    }
}