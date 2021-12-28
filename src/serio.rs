/** 
 * This module represents the serial communication protocol
 * based on UART physical hardware. For simplicity, it is tightly
 * coupled to a specific uart device.
*/
use crate::phys::uart::*;

pub fn serio_init() {

}

pub fn serio_baud() {

}

pub fn serio_write(string: &[u8]) {
    let mut idx = 0;
    while idx < string.len() {
        serio_write_byte(string[idx]);
        idx += 1;
    }
}

pub fn serio_write_byte(byte: u8) {

}