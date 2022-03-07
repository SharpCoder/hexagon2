use crate::*;
use crate::http::models::HttpRequest;
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
const EN_PIN: usize = 3;

static mut INITIALIZED: bool = false;
static mut INITIALIZE_TIMEOUT: u64 = 0;
static mut PROCESS_COMPLETE: bool = false;
static mut PROCESS_TIMEOUT: u64 = 0;

static mut OK: Option<Str> = None;
static mut READY: Option<Str> = None;
static mut ERROR: Option<Str> = None;
static mut FAIL: Option<Str> = None;
static mut CLOSED: Option<Str> = None;
static mut SEND_OK: Option<Str> = None;

// Buffers to hold the content and header, so we don't have
// to recreate them all the time.
static mut CONTENT: Option<Str> = None;
static mut HEADER: Option<Str> = None;

pub struct WifiTask {
    pub ready: bool,
}

impl WifiTask {

    pub fn new() -> Self {
        return WifiTask {
            ready: false,
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

        pin_mode(EN_PIN, Mode::Output);
        pin_pad_config(EN_PIN, PadConfig { 
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
            CLOSED = Some(str!(b"CLOSED"));
            SEND_OK = Some(str!(b"SEND OK"));
            HEADER = Some(Str::new());
            CONTENT = Some(Str::new());
            INITIALIZE_TIMEOUT = nanos() + S_TO_NANO * 30;
        }
    }

    pub fn system_loop(&mut self) {
        if unsafe { INITIALIZED } {
            self.ready = true;
        }

        gate_open!()
            .once(|| {
                unsafe { INITIALIZED = false; }
                pin_out(EN_PIN, Power::Low);
                teensycore::wait_ns(30 * teensycore::MS_TO_NANO);
                pin_out(EN_PIN, Power::High);
                
                pin_out(RST_PIN, Power::Low);
                teensycore::wait_ns(30 * teensycore::MS_TO_NANO);
                pin_out(RST_PIN, Power::High);
                teensycore::wait_ns(30 * teensycore::MS_TO_NANO);
                // esp8266_reset(DEVICE);
                pin_mode(RST_PIN, Mode::Input);
                teensycore::wait_ns(300 * teensycore::MS_TO_NANO);
                debug::debug_str(b"reset");
            })
            .when(ready, || {
                esp8266_configure_echo(DEVICE, true);
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
                esp8266_multiple_connections(DEVICE, false);
            })
            // .when(ok, || {
            //     // DNS Lookup
            //     let mut addr = str!(b"worldtimeapi.org");
            //     esp8266_dns_lookup(DEVICE, &addr);
            //     addr.drop();
            // })
            .when(ok, || {
                // Parse the response
                let content = serial_read(DEVICE);
                let mut ip = str!(b"213.188.196.246");// parse_ip(content);
                
                // Request world time
                esp8266_open_tcp(DEVICE, &ip, 80, None);
                ip.drop();
            })
            .when(ok, || {
                // Generate the http request
                let mut request = HttpRequest {
                    method: str!(b"GET"),
                    request_path: str!(b"/api/timezone/America/Los_Angeles.txt"),
                    host: str!(b"worldtimeapi.org"),
                    headers: None,
                    content: None,
                };

                // Send request
                esp8266_write(DEVICE, &request.to_str(), None);
                request.drop();
            })
            .when(send_ok, || {
                unsafe { INITIALIZED = true };
                // Parse response here
                
            })
            .when(|gate| {
                return false;
            }, || {
                
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
fn closed(gate: &mut Gate) -> bool { return rx_contains(gate, unsafe { &CLOSED }, true); }
fn send_ok(gate: &mut Gate) -> bool { return rx_contains(gate, unsafe { &SEND_OK }, false); }
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

