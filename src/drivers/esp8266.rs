//! This module is a driver for the ESP8266 WiFi
//! peripheral. It includes wrappers for most
//! of the AT+ instruction set.

use teensycore::*;
use teensycore::serio::*;
use teensycore::math::*;
use teensycore::system::str::*;

pub enum WifiMode {
    Client = 0x1,
    Host = 0x2,
    All = 0x3,
}

/// The AT command checks whether the system is
/// in a healthy state.
pub fn esp8266_at(device: SerioDevice) {
    esp8266_send(device, b"AT");
}

/// Sends the AT+RST reset command, causing
/// the system to do a software-level
/// reboot.
pub fn esp8266_reset(device: SerioDevice) {
    esp8266_send(device, b"AT+RST");
}

/// Configures the ESP8266 to either send back
/// received commands, or not.
pub fn esp8266_configure_echo(device: SerioDevice, enabled: bool) {
    match enabled {
        true => {
            esp8266_send(device, b"ATE1");
        },
        false => {
            esp8266_send(device, b"ATE0");
        }
    }
}

/// Configure the ESP8266 to either be a client,
/// a host, or both.
pub fn esp8266_wifi_mode(device: SerioDevice, mode: WifiMode) {
    serial_write(device, b"AT+CWMODE=");
    serial_write(device, &[int_to_hex(mode as u8)]);
    serial_write(device, b"\r\n");
}

/// Connect to a wifi access point.
pub fn esp8266_connect_to_wifi(device: SerioDevice, ssid: Str, pwd: Str) {
    serial_write(device, b"AT+CWJAP=\"");
    serial_write_str(device, &ssid);
    serial_write(device, b"\",\"");
    serial_write_str(device, &pwd);
    serial_write(device, b"\"\r\n");
}

/// Disconnect from any currently active access point
pub fn esp8266_disconnect_from_wifi(device: SerioDevice) {
    esp8266_send(device, b"AT+CWQAP");
}

/// Given a domain, this command will return the ip address
pub fn esp8266_dns_lookup(device: SerioDevice, domain: Str) {
    serial_write(device, b"AT+CIPDOMAIN=\"");
    serial_write_str(device, &domain);
    serial_write(device, b"\"\r\n");
}

pub fn esp8266_connect(device: SerioDevice, domain: Str) {
    serial_write(device, b"AT+CIPSTART=\"");
    serial_write_str(device, &domain);
    serial_write(device, b"\"\r\n");
}

pub fn esp8266_write(device: SerioDevice, content: Str) {
    serial_write(device, b"AT+CIPSEND=");
    // serial_write()
    todo!("implement");
}
/// Send a raw command to the esp8266
fn esp8266_send(device: SerioDevice, command: &[u8]) {
    serial_write(device, command);
    serial_write(device, b"\r\n");
}