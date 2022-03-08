use crate::*;
use crate::models::SystemCommand;
use teensycore::*;
use teensycore::debug::*;
use teensycore::math::atoi;
use teensycore::system::str::*;
use teensycore::system::vector::*;

// This buffer is shared amongs all string operations. It will
// be frequently cleared and should never be relied upon for
// any kind of persisted data. It is simply a reservoire of
// memory.
static mut BUFFER: Option<Str> = None;
static mut LINE: Option<Str> = None;

// The system command delimiter
const DELIMITER: u8 = b';';

/// HttpResponse is a tuple containing two primary pieces
/// of information. The first parameter is the parsed header
/// and the third parameter is the parsed content.
pub type HttpResponse = (Option<Str>, Option<Str>);

enum ParserState {
    LookForStart = 0x0,
    LookForLength = 0x1,
    LookForContent = 0x2,
    ReadContent = 0x3,
    Done = 0x4,
}

fn init_buffer<'a>() -> (&'a mut Str, &'a mut Str) {
    unsafe {
        match BUFFER {
            None => {
                BUFFER = Some(Str::new());
                LINE = Some(Str::new());
            },
            Some(_) => { }
        }

        let buffer = BUFFER.as_mut().unwrap();
        buffer.clear();

        let line = LINE.as_mut().unwrap();
        line.clear();

        return (buffer, line);
    }
}

pub fn parse_http_request(rx_buffer: &Str, header: &mut Str, content: &mut Str) -> bool {
    // Ensure buffers are setup
    init_buffer();
    header.clear();
    content.clear();

    debug_str(b"HELLO YES THIS IS DOG");
    
    let buffer = unsafe { BUFFER.as_mut().unwrap() };
    let line = unsafe { LINE.as_mut().unwrap() };

    let mut content_length_cmp = str!(b"Content-Length: ");
    let mut ipd = str!(b"+IPD,");
    let mut colon = str!(b":");
    let mut crnl = str!(b"\r\n");
    

    let mut content_length: Option<u64> = None;
    let mut state = ParserState::LookForStart;
    let mut bytes_read = 0;
    let mut parsed_packet = parse_response_payload(&rx_buffer);

    // Debug each line of parsed data.
    // Search for the content length
    // Then search for the content start
    // Then read until content length
    let mut lines = parsed_packet.split(b'\n');
    for line in lines.into_iter() {

        match state {
            ParserState::LookForStart => {
                if line.contains(&content_length_cmp) {
                    let mut temp = line.slice(content_length_cmp.len(), line.len());
                    content_length = Some(atoi(&temp));
                    temp.drop();

                    state = ParserState::LookForContent;
                }

                header.join(&line);

            },
            ParserState::LookForContent => {
                // Look for the blank line
                if line.len() == 1 {
                    state = ParserState::ReadContent;
                } else {
                    header.join(&line);
                }
            },
            ParserState::ReadContent => {
                content.join(&line);
                content.append(b"\n");

                if content.len() >= content_length.unwrap() as usize {
                    state = ParserState::Done;
                }
            },
            _ => {

            }
        }
    }

    lines.free();
    ipd.drop();
    colon.drop();
    crnl.drop();
    content_length_cmp.drop();
    parsed_packet.drop();

    // Checksum validation
    if state as usize == ParserState::Done as usize {
        debug_str(b"");
        debug_u64(content.len() as u64, b"content-length");
        return true;
    } else {
        debug_str(b"");
        debug_u64(content.len() as u64, b"content-length");
        header.clear();
        content.clear();
        return false;
    }
}

/// This method takes a serial blob of data and returns
/// All content after IPD+,[content_length]: and is smart
/// enough to handle chunking.
pub fn parse_response_payload(buf: &Str) -> Str {
    let mut result = Str::new();
    let mut ipd = str!(b"+IPD,");
    let mut colon = str!(b":");


    let mut state: ParserState = ParserState::LookForStart;
    let mut temp = Str::new();
    let mut ipd_buf = Vector::<u8>::new();
    let mut packet_length: Option<usize> = None;

    // Scan until ipd is found. At which point,
    // read until packet_length is known at which point
    // aggregate packet details. Repeat until end of stream.
    for char in buf.into_iter() {
        ipd_buf.enqueue(char);
        if ipd_buf.size() > 5 {
            ipd_buf.dequeue();
        }

        match state {
            ParserState::LookForLength => {
                if char == b':' {
                    packet_length = Some(atoi(&temp) as usize);
                    state = ParserState::ReadContent;
                    temp.clear();
                } else {
                    temp.append(&[char]);
                }
            },
            ParserState::ReadContent => {
                temp.append(&[char]);
                if packet_length.is_some() && temp.len() >= packet_length.unwrap() {
                    result.join(&temp);
                    temp.clear();
                    state = ParserState::LookForStart;
                }
            },
            _ => {

            }
        }

        // Check for ipd
        if arr_contains(ipd_buf, &ipd) {
            state = ParserState::LookForLength;
            temp.clear();
        }

    }
    debug_str(b"");
    debug_u64(packet_length.unwrap_or(0) as u64, b"packet-length");

    ipd_buf.free();
    temp.drop();
    ipd.drop();
    colon.drop();
    return result;
}

/// Take a content string and return the parsed system command.
pub fn parse_command(buf: &Str) -> SystemCommand {
    let (buffer, _) = init_buffer();
    let mut ptr = 0;
    let mut reg = 0;
    let mut result = SystemCommand::new();

    for char in buf.into_iter() {
        if ptr < 4 {
            result.command[ptr] = char;
        } else if ptr == 4 && char == DELIMITER {
            // Skip  
        } else if char == DELIMITER {
            // Process
            if reg < result.args.len() {

                // Only process if we have data
                if buffer.len() > 0 {
                    let number = atoi(&buffer);
                    let is_negative = match buffer.char_at(0) {
                        None => false,
                        Some(char) => char == b'-',
                    };

                    result.args[reg] = number as i32;
                    if is_negative {
                        result.args[reg] *= -1;
                    }
                }
                
                reg += 1;
            }

            buffer.clear();
        } else {
            buffer.append(&[char]);
        }

        ptr += 1;
    }


    return result;
}

fn arr_contains(arr: Vector<u8>, target: &Str) -> bool {
    if arr.size() < target.len() {
        return false;
    }

    for idx in 0 .. target.len() {
        let c = arr.get(idx);
        if c.is_some() && c.unwrap() != target.char_at(idx).unwrap() {
            return false;
        }
    }

    return true;
}