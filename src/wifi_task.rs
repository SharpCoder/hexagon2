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

static mut OK: Option<Str> = None;
static mut READY: Option<Str> = None;
static mut ERROR: Option<Str> = None;
static mut FAIL: Option<Str> = None;
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
            })
            .sealed()
            .compile();

        gate_open!()
            .when_nano(MS_TO_NANO * 1500, || {
                let (header, content) = parse_http_request(serial_read(DEVICE));
                if content.is_some() {
                    debug_str(b"=== found content ===");
                    serial_write_str(SerioDevice::Debug, &Str::from_str(&content.as_ref().unwrap()));
                    serial_read(DEVICE).clear();
                    
                    header.unwrap().drop();
                    content.unwrap().drop();
                    esp8266_close_tcp(DEVICE, Some(0));
                }
            })
            .compile();
    }
}

fn ready(gate: &mut Gate) -> bool { return rx_contains(gate, unsafe { &READY }); }
fn ok(gate: &mut Gate) -> bool { return rx_contains(gate, unsafe { &OK }); }
fn rx_contains(gate: &mut Gate, cond: &Option<Str>) -> bool {
    match unsafe { &ERROR } {
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