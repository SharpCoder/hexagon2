use teensycore::*;
use teensycore::clock::nanos;
use teensycore::{phys::pins::*, wait_ns, MICRO_TO_NANO};

pub struct MSGEQ7Driver {
    clk_pin: usize,
    strobe_pin: usize,
    out_pin: usize,
    reset_pin: usize,
}

impl MSGEQ7Driver {
    pub fn new(clk_pin: usize, strobe_pin: usize, out_pin: usize, reset_pin: usize) -> Self {

        // Configure all the pins   
        pin_mode(strobe_pin, Mode::Output);
        pin_mode(reset_pin, Mode::Output);
        pin_mode(out_pin, Mode::Input);

        pin_mux_config(reset_pin, Alt::Alt5);
        pin_mux_config(strobe_pin, Alt::Alt5);
        pin_mux_config(out_pin, Alt::Alt5);


        pin_pad_config(out_pin, PadConfig { 
            hysterisis: false, 
            resistance: PullUpDown::PullDown100k, 
            pull_keep: PullKeep::Pull, 
            pull_keep_en: false, 
            open_drain: false, 
            speed: PinSpeed::Max200MHz, 
            drive_strength: DriveStrength::Max, 
            fast_slew_rate: true 
        });
        
        pin_pad_config(reset_pin, PadConfig { 
            hysterisis: false, 
            resistance: PullUpDown::PullDown100k, 
            pull_keep: PullKeep::Pull, 
            pull_keep_en: false, 
            open_drain: false, 
            speed: PinSpeed::Max200MHz, 
            drive_strength: DriveStrength::Max, 
            fast_slew_rate: true 
        });

        pin_pad_config(strobe_pin, PadConfig { 
            hysterisis: false, 
            resistance: PullUpDown::PullDown100k, 
            pull_keep: PullKeep::Pull, 
            pull_keep_en: false, 
            open_drain: false, 
            speed: PinSpeed::Max200MHz, 
            drive_strength: DriveStrength::Max, 
            fast_slew_rate: true 
        });

        return MSGEQ7Driver {
            clk_pin: clk_pin,
            strobe_pin: strobe_pin,
            out_pin: out_pin,
            reset_pin: reset_pin,
        };
    }

    pub fn read(&self) -> [u64;7] {
        return msgeq7_read(self.reset_pin, self.strobe_pin, self.out_pin);
    }
}

fn msgeq7_reset(reset_pin: usize) {
    pin_out(reset_pin, Power::High);
    wait_ns(100);
    pin_out(reset_pin, Power::Low);
}

fn msgeq7_strobe(strobe_pin: usize) {
    pin_out(strobe_pin, Power::High);
    wait_ns(18 * MICRO_TO_NANO);
    pin_out(strobe_pin, Power::Low);
}

/// Poll until the output goes low
fn msgeq7_read_one(output_pin: usize) -> u64 {
    let start = nanos();
    let timeout: u64 = start + 72 * MICRO_TO_NANO;
    let mut high = 0;

    loop {
        let now = nanos();
        if now > timeout {
            break;
        }

        if pin_read(output_pin) > 0 {
            high += 1;
        } 
    }

    return high;
}

pub fn msgeq7_read(reset_pin: usize, strobe_pin: usize, output_pin: usize) -> [u64; 7] {
    let mut result = [0; 7];
    let mut index = 0;

    msgeq7_reset(reset_pin);
    
    loop {
        msgeq7_strobe(strobe_pin);

        let now = nanos();
        let target = now + 72 * MICRO_TO_NANO;
        result[index] = msgeq7_read_one(output_pin);

        if nanos() < target {
            wait_ns(nanos() - target);
        }

        index += 1;
        if index == 7 {
            break;
        }
    }

    return result;
}