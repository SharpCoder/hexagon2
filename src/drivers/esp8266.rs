//! This module is a driver for the ESP8266 WiFi
//! peripheral. It includes wrappers for most
//! of the AT+ instruction set.

use teensycore::*;
use teensycore::serio::*;
use teensycore::clock::*;
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
    esp8266_raw(device, b"AT");
}

/// Sends the AT+RST reset command, causing
/// the system to do a software-level
/// reboot.
pub fn esp8266_reset(device: SerioDevice) {
    esp8266_raw(device, b"AT+RST");
}

/// Configures the ESP8266 to either send back
/// received commands, or not.
pub fn esp8266_configure_echo(device: SerioDevice, enabled: bool) {
    match enabled {
        true => {
            esp8266_raw(device, b"ATE1");
        },
        false => {
            esp8266_raw(device, b"ATE0");
        }
    }
}

pub fn esp8266_dhcp_mode(device: SerioDevice, mode: WifiMode, dhcp_enabled: bool) {
    // serial_write(device, b"AT+CWDHCP=1");
    serial_write_str(device, &itoa((mode as u64) - 1));
    serial_write(device, b",");
    match dhcp_enabled {
        true => {
            serial_write(device, b"0\r\n");
        },
        false => {
            serial_write(device, b"1\r\n");
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
pub fn esp8266_connect_to_wifi(device: SerioDevice, ssid: &Str, pwd: &Str) {
    serial_write(device, b"AT+CWJAP=\"");
    serial_write_str(device, &ssid);
    serial_write(device, b"\",\"");
    serial_write_str(device, &pwd);
    serial_write(device, b"\"\r\n");
}

/// Disconnect from any currently active access point
pub fn esp8266_disconnect_from_wifi(device: SerioDevice) {
    esp8266_raw(device, b"AT+CWQAP");
}

/// Given a domain, this command will return the ip address
pub fn esp8266_dns_lookup(device: SerioDevice, domain: &Str) {
    serial_write(device, b"AT+CIPDOMAIN=\"");
    serial_write_str(device, &domain);
    serial_write(device, b"\"\r\n");
}

/// Esetablish a TCP connection
pub fn esp8266_open_tcp(device: SerioDevice, domain: &Str, id: Option<u8>) {
    match id {
        None => {
            serial_write(device, b"AT+CIPSTART=\"");
        },
        Some(con_id) => {
            serial_write(device, b"AT+CIPSTART=");
            serial_write_str(device, &itoa(con_id as u64));
            serial_write(device, b",\"");
        }
    }
    serial_write_str(device, &domain);
    serial_write(device, b"\"\r\n");
}

pub fn esp8266_version(device: SerioDevice) {
    esp8266_raw(device, b"AT+GMR");
}

/// Write content over TCP/UDP
pub fn esp8266_write(device: SerioDevice, content: Str, id: Option<u8>) {
    serial_write(device, b"AT+CIPSEND=");

    match id {
        None => {

        },
        Some(con_id) => {
            serial_write_str(device, &itoa(con_id as u64));
            serial_write(device, b",");
        }
    }

    serial_write_str(device, &itoa(content.len() as u64));
    serial_write(device, b"\r\n");
    // TODO: would be great to not block at all...
    esp8266_block_until(device, b"OK", S_TO_NANO);
    serial_write_str(device, &content);
    serial_write(device, b"\r\n");
}

/// Close active TCP connection
pub fn esp8266_close_tcp(device: SerioDevice, id: Option<u8>) {
    serial_write(device, b"AT+CIPCLOSE");
    match id {
        None => {
            serial_write(device, b"\r\n");
        },
        Some(con_id) => {
            serial_write(device, b"=");
            serial_write_str(device, &itoa(con_id as u64));
            serial_write(device, b"\r\n");
        }
    }
}

/// Get the devices ip address
pub fn esp8266_read_ip(device: SerioDevice) {
    esp8266_raw(device, b"AT+CIFSR");
}

pub fn esp8266_set_ip(device: SerioDevice, ip: Str) {
    serial_write(device, b"AT+CIPAP=\"");
    serial_write_str(device, &ip);
    serial_write(device, b"\"\r\n");
}

/// Set whether the device will automatically attempt to reconnect
/// to the AP on boot.
pub fn esp8266_auto_connect(device: SerioDevice, auto_connect: bool) {
    match auto_connect {
        true => {
            esp8266_raw(device, b"AT+CWAUTOCONN=1");
        },
        false => {
            esp8266_raw(device, b"AT+CWAUTOCONN=0");
        },
    }
}

/// This muxes the device to either allow or disallow multiple connections.
/// If multiple connections are allowed, you'll need to be cognizant
/// of that when interfacing with some of the other commands.
pub fn esp8266_multiple_connections(device: SerioDevice, allow: bool) {
    match allow {
        true => {
            esp8266_raw(device, b"AT+CIPMUX=1");
        },
        false => {
            // This may require a reboot to work as intended
            esp8266_raw(device, b"AT+CIPMUX=0");
        }
    }
}

pub fn esp8266_create_server(device: SerioDevice, port: u32) {
    serial_write(device, b"AT+CIPSERVER=1,");
    serial_write_str(device, &itoa(port as u64));
    serial_write(device, b"\r\n");
}

/// Set the server timeout in seconds
pub fn esp8266_set_server_timeout(device: SerioDevice, timeout: u32) {
    serial_write(device, b"AT+CIPSTO=");
    serial_write_str(device, &itoa(timeout as u64));
    serial_write(device, b"\r\n");
}

pub fn esp8266_list_wifi(device: SerioDevice) {
    esp8266_raw(device, b"AT+CWLAP");
}

pub fn esp8266_block_until(device: SerioDevice, command: &[u8], timeout: u64) {
    let threshold = nanos() + timeout;
    let cmd = str!(command);

    loop {
        let buf = serial_read(device);
        if buf.contains(&cmd) {
            buf.clear();
            wait_ns(MS_TO_NANO * 500);
            return;
        } else if nanos() > threshold {
            return;
        }
        wait_ns(MS_TO_NANO * 100);
    }
}

/// Send a raw command to the esp8266
pub fn esp8266_raw(device: SerioDevice, command: &[u8]) {
    serial_write(device, command);
    serial_write(device, b"\r\n");
}