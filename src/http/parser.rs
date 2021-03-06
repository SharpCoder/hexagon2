use teensycore::*;
use teensycore::debug::*;
use teensycore::math::atoi;
use teensycore::system::str::*;
use teensycore::system::vector::*;

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


pub fn parse_http_request(rx_buffer: &Str, header: &mut Str, content: &mut Str) -> bool {
    // Ensure buffers are setup
    header.clear();
    content.clear();

    debug_str(b"HELLO YES THIS IS DOG");
    
    let mut content_length_cmp = str!(b"Content-Length: ");
    let mut ipd = str!(b"+IPD,");
    let mut colon = str!(b":");
    let mut crnl = str!(b"\r\n");
    

    let mut content_length: Option<u64> = None;
    let mut state = ParserState::LookForStart;
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
        return true;
    } else {
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

    ipd_buf.free();
    temp.drop();
    ipd.drop();
    colon.drop();
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