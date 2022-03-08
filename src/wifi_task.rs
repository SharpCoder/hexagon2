use crate::*;
use crate::http::models::HttpRequest;
use crate::http::parser::*;
use crate::pixel_engine::shader_config::ShaderConfig;
use crate::pixel_engine::shader_config::ShaderConfigList;
use teensycore::*;
use teensycore::gate::*;
use teensycore::system::str::*;
use teensycore::system::vector;
use teensycore::system::vector::*;
use teensycore::serio::*;
use teensycore::phys::pins::*;
use teensycore::debug::*;
use teensycore::math::atoi;
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
static mut BAD_REQUEST: Option<Str> = None;

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
            BAD_REQUEST = Some(str!(b"Bad Request"));
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
                teensycore::wait_ns(100 * teensycore::MS_TO_NANO);
                debug::debug_str(b"reset");
                esp8266_version(DEVICE);
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
            .when(ok, || {
                // Parse the response
                let content = serial_read(DEVICE);
                let mut ip = str!(b"52.27.143.19");                
                // Request world time
                esp8266_open_tcp(DEVICE, &ip, 80, None);
                ip.drop();
            })
            .when(ok, || {
                // Generate the http request
                let mut request = HttpRequest {
                    method: str!(b"GET"),
                    request_path: str!(b"/hexwall"),
                    host: str!(b"amazon.com"),
                    headers: None,
                    content: None,
                };

                // Send request
                esp8266_write(DEVICE, &request.to_str(), None);
                request.drop();
            })
            .when(send_ok, || { })
            .when_nano(2000 * MS_TO_NANO,  || { })
            .when(|gate| {
                // Check for bad request
                let buf = serial_read(DEVICE);
                match unsafe { &BAD_REQUEST } {
                    None => {
                        return false;
                    },
                    Some(target) => {
                        if buf.contains(&target) {
                            gate.reset();
                            buf.clear();
                            return false;
                        }
                    }
                }
                
                // Parse response here
                let shaders = parse_config(buf);
                if shaders.size() > 0 {
                    buf.clear();
                    set_shader_configs(shaders);
                    return true;
                } else {
                    gate.reset();
                    buf.clear();
                    return false;
                }
            }, || {
                unsafe { INITIALIZED = true };
            })
            .when(|gate| {
                return false;
            }, || {
                
            })
            .sealed()
            .compile();
    }
}

fn parse_config(serial_content: &Str) -> ShaderConfigList {
    // Parse the http headers
    let mut time_cmd = str!(b"time");
    let mut rule_cmd = str!(b"rule");
    let mut header = Str::new();
    let mut content = Str::new();
    let mut configs = Vector::new();

    if parse_http_request(serial_content, &mut header, &mut content) {
        let mut lines = content.split(b'\n');
        for line in lines.into_iter() {
            let mut paths = line.split(b';');
            match paths.get(0) {
                None => {},
                Some(command) => {

                    if command.contains(&time_cmd) && paths.size() > 1 {
                        let epoch = atoi(&paths.get(1).unwrap()) / 1000;
                        set_world_time(epoch);
                    } else if command.contains(&rule_cmd) && paths.size() > 3 {
                        let config = ShaderConfig { 
                            time_range_start: atoi(&paths.get(1).unwrap()), 
                            time_range_end: atoi(&paths.get(2).unwrap()), 
                            shader: paths.get(3).unwrap(), 
                            probability: atoi(&paths.get(4).unwrap()),
                        };

                        configs.push(config);
                    }
                }
            }
            paths.free();

        }
        lines.free();
    }

    time_cmd.drop();
    rule_cmd.drop();
    header.drop();
    content.drop();

    return ShaderConfigList {
        configs: configs,
    }
}

fn parse_packet_from_serial(serial_content: &Str) -> Str {
    // Parse the serial_content until you encounter one single packet and then consume it
    let mut result = Str::new();
    let mut begin_target = str!(b"+IPD,");
    let mut colon = str!(b":");
    match serial_content.index_of(&begin_target) {
        None => {},
        Some(start_idx) => {

            // We know where the content begins. Now read until we hit the colon
            let mut packet_size_buff = Str::new();
            let mut packet_size = None;
            let mut substr = serial_content.slice(start_idx, serial_content.len());
            
            for char in substr.into_iter() {
                if char == b':' {
                    packet_size = Some(atoi(&packet_size_buff) as usize);
                } else if packet_size.is_some() {
                    result.append(&[char]);
                    if result.len() >= packet_size.unwrap() {
                        break;
                    }
                } else {
                    packet_size_buff.append(&[char]);
                }
            }


            substr.drop();

        }
    }

    return result;
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

    match unsafe { &BAD_REQUEST } {
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

