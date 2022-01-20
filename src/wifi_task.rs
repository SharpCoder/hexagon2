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
const RST_PIN: usize = 3;

static mut INITIALIZED: bool = false;
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
        }
    }



    pub fn system_loop(&mut self) {
        gate_open!()
            .once(|| {
                pin_out(RST_PIN, Power::Low);
                pin_out(RST_PIN, Power::High);
                esp8266_reset(DEVICE);
            })
            .when(ready, || {
                esp8266_auto_connect(DEVICE, false);
            })
            .when(ok, || {
                esp8266_configure_echo(DEVICE, true);
            })
            .when(ok, || {
                esp8266_wifi_mode(DEVICE, WifiMode::Client);
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
                esp8266_read_ip(DEVICE);
            })
            .when(ok, || {
                esp8266_multiple_connections(DEVICE, true);
            })
            .when(ok, || {
                esp8266_create_server(DEVICE, 80);
            })
            .when(ok, || {
                // Gate complete. This empty segment is meant to verify
                // the final command succeded. Because if we get here
                // we're good. If we don't get here, the watchdog
                // mechanism should have kicked in and rebooted the 
                // esp32.
                unsafe {
                    INITIALIZED = true;
                }
            })
            .sealed()
            .compile();

        gate_open!()
            .when(|_| {
                if unsafe { !INITIALIZED } {
                    return false;
                }

                let header = match unsafe { HEADER.as_mut() } {
                    None => {
                        return false;
                    },
                    Some(value) => value
                };

                let content = match unsafe { CONTENT.as_mut() } {
                    None => {
                        return false;
                    },
                    Some(value) => value,
                };

                return parse_http_request(serial_read(DEVICE), header, content);
            }, || {
                // Process content

                unsafe {
                    PROCESS_COMPLETE = true;
                    PROCESS_TIMEOUT = nanos() + S_TO_NANO * 3;
                    esp8266_close_tcp(DEVICE, Some(0));
                }
            })
            .compile();

            gate_open!()
                .when(|_| {
                    return unsafe { INITIALIZED && PROCESS_COMPLETE } && (
                        nanos() > unsafe { PROCESS_TIMEOUT } ||  err_or_ok(serial_read(DEVICE))
                    );
                }, || {
                    serial_read(DEVICE).clear();
                    unsafe {
                        PROCESS_COMPLETE = false;
                    }
                })
                .compile();
    }
}

fn ready(gate: &mut Gate) -> bool { return rx_contains(gate, unsafe { &READY }); }
fn ok(gate: &mut Gate) -> bool { return rx_contains(gate, unsafe { &OK }); }
fn rx_contains(gate: &mut Gate, cond: &Option<Str>) -> bool {
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
                serial_read(DEVICE).clear();
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
