use crate::*;
use crate::http::parser::*;
use teensycore::*;
use teensycore::gate::*;
use teensycore::system::str::*;
use teensycore::serio::*;
use teensycore::phys::pins::*;
use teensycore::debug::*;
use crate::drivers::esp8266::*;

const DEVICE: SerioDevice = SerioDevice::Default;
const RST_PIN: usize = 2;

static mut INITIALIZED: bool = false;
static mut INITIALIZE_TIMEOUT: u64 = 0;
static mut PROCESS_COMPLETE: bool = false;
static mut PROCESS_TIMEOUT: u64 = 0;

static mut OK: Option<Str> = None;
static mut READY: Option<Str> = None;
static mut ERROR: Option<Str> = None;
static mut FAIL: Option<Str> = None;

// Buffers to hold the content and header, so we don't have
// to recreate them all the time.
static mut CONTENT: Option<Str> = None;
static mut HEADER: Option<Str> = None;

pub struct WifiTask {

}

impl WifiTask {

    pub fn new() -> Self {
        return WifiTask {

        };
    }

    pub fn init(&mut self) {
        serial_init(DEVICE);
        serial_baud(DEVICE, 115200);

        // Restart
        pin_mode(RST_PIN, Mode::Output);
        pin_pad_config(RST_PIN, PadConfig { 
            hysterisis: false, 
            resistance: PullUpDown::PullDown100k, 
            pull_keep: PullKeep::Pull, 
            pull_keep_en: true, 
            open_drain: false, 
            speed: PinSpeed::Max200MHz, 
            drive_strength: DriveStrength::Max, 
            fast_slew_rate: false 
        });

        // If we don't cache these, it uses up a lot of memory re-creating them
        // in the main loop. Which just seems silly.
        unsafe {
            OK = Some(str!(b"OK"));
            READY = Some(str!(b"ready"));
            ERROR = Some(str!(b"ERROR"));
            FAIL = Some(str!(b"FAIL"));
            HEADER = Some(Str::new());
            CONTENT = Some(Str::new());
            INITIALIZE_TIMEOUT = nanos() + S_TO_NANO * 30;
        }
    }

    pub fn system_loop(&mut self) {
        gate_open!()
            .once(|| {
                unsafe { INITIALIZED = false; }
                pin_out(RST_PIN, Power::Low);
                teensycore::wait_ns(100 * teensycore::MS_TO_NANO);
                pin_out(RST_PIN, Power::High);
                teensycore::wait_ns(100 * teensycore::MS_TO_NANO);
                esp8266_reset(DEVICE);
                pin_mode(RST_PIN, Mode::Input);
                debug::debug_str(b"reset");
            })
            .when(ready, || {
                esp8266_version(DEVICE);
                // esp8266_configure_echo(DEVICE, true);
            })
            .when(ok, || {
                esp8266_wifi_mode(DEVICE, WifiMode::Client);
            })
            .when(ok, || {
                esp8266_auto_connect(DEVICE, false);
            })
            .when(ok, || {
                let mut ssid = str!(b"NCC-1701D");
                let mut pwd = str!(b"password_here");

                esp8266_connect_to_wifi(DEVICE, &ssid, &pwd);

                // Free up memory
                ssid.drop();
                pwd.drop();
            })
            .when(ok, || {
                esp8266_multiple_connections(DEVICE, true);
            })
            .when(ok, || {
                // DNS Lookup
                let mut addr = str!(b"worldtimeapi.org");
                esp8266_dns_lookup(DEVICE, &addr);
                addr.drop();
            })
            .when(ok_without_clear, || {
                // Parse the response
                let content = serial_read(DEVICE);
                let mut ip = parse_ip(content);
                
                // Clear serial buffer
                serial_read(DEVICE).clear();

                // Request world time
                esp8266_open_tcp(DEVICE, &ip, None);

                ip.drop();
            })
            .sealed()
            .compile();

        // gate_open!()
        //     .when(|_| {
        //         if unsafe { !INITIALIZED } {
        //             return false;
        //         }

        //         let header = match unsafe { HEADER.as_mut() } {
        //             None => {
        //                 return false;
        //             },
        //             Some(value) => value
        //         };

        //         let content = match unsafe { CONTENT.as_mut() } {
        //             None => {
        //                 return false;
        //             },
        //             Some(value) => value,
        //         };

        //         return parse_http_request(serial_read(DEVICE), header, content);
        //     }, || {
        //         // Process content
        //         match unsafe { CONTENT.as_mut() } {
        //             None => {},
        //             Some(content) => {
        //                 let command = parse_command(content);
        //                 proc_emit(&command);
        //             }
        //         }
        //         unsafe {
        //             PROCESS_COMPLETE = true;
        //             PROCESS_TIMEOUT = nanos() + S_TO_NANO * 3;
        //             esp8266_close_tcp(DEVICE, Some(0));
        //             serial_read(DEVICE).clear();
        //         }
        //     })
        //     .compile();

        //     // gate_open!()
        //     //     .when(|_| {
        //     //         return unsafe { INITIALIZED && PROCESS_COMPLETE } && (
        //     //             nanos() > unsafe { PROCESS_TIMEOUT } ||  err_or_ok(serial_read(DEVICE))
        //     //         );
        //     //     }, || {
        //     //         serial_read(DEVICE).clear();
        //     //         unsafe {
        //     //             PROCESS_COMPLETE = false;
        //     //         }
        //     //     })
        //     //     .compile();
    }
}

fn parse_ip(str: &Str) -> Str {
    let mut target = str!(b":");
    let mut newline = str!(b"\n");
    let mut ip = Str::new();

    if str.contains(&target) {
        let mut ip_start = str.slice(
            str.index_of(&target).unwrap() + 1,
            str.len()
        );
        
        if ip_start.contains(&newline) {
            ip.join_with_drop(&mut ip_start.slice(
                0,
                ip_start.index_of(&newline).unwrap() - 1
            ));

        }

        ip_start.drop();
    }
    

    target.drop();
    newline.drop();

    return ip;
}

fn ready(gate: &mut Gate) -> bool { return rx_contains(gate, unsafe { &READY }, true); }
fn ok(gate: &mut Gate) -> bool { return rx_contains(gate, unsafe { &OK }, true); }
fn ok_without_clear(gate: &mut Gate) -> bool { return rx_contains(gate, unsafe { &OK }, false); }
fn rx_contains(gate: &mut Gate, cond: &Option<Str>, clear: bool) -> bool {
    match unsafe { &ERROR } {
        // I know you think this logic is wrong, but it's not
        None => {
            return false;
        },
        Some(target) => {
            if serial_read(DEVICE).contains(&target) {
                gate.reset();
                serial_read(DEVICE).clear();
                return false;
            }
        }
    }

    match unsafe { &FAIL } {
        // I know you think this logic is wrong, but it's not
        None => {
            return false;
        },
        Some(target) => {
            if serial_read(DEVICE).contains(&target) {
                gate.reset();
                serial_read(DEVICE).clear();
                return false;
            }
        }
    }

    match cond {
        None => {
            return false;
        },
        Some(target) => {
            if serial_read(DEVICE).contains(&target) {
                if clear {
                    serial_read(DEVICE).clear();
                }
                return true;
            } else {
                return false;
            }
        }
    }
}

fn err_or_ok(rx_buffer: &Str) -> bool {
    match unsafe { &FAIL } {
        None => { return false; },
        Some(fail) => {
            if rx_buffer.contains(fail) {
                return true;
            }
        }
    }

    match unsafe { &ERROR } {
        None => { return false; },
        Some(error) => {
            if rx_buffer.contains(error) {
                return true;
            }
        }
    }

    match unsafe { &OK } {
        None => { return false; },
        Some(ok) => {
            if rx_buffer.contains(ok) {
                return true;
            }
        }
    }

    return false;
}

