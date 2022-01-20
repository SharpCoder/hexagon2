use crate::*;
use teensycore::*;
use teensycore::debug::*;
use teensycore::math::atoi;
use teensycore::system::str::*;

// This buffer is shared amongs all string operations. It will
// be frequently cleared and should never be relied upon for
// any kind of persisted data. It is simply a reservoire of
// memory.
static mut BUFFER: Option<Str> = None;
static mut LINE: Option<Str> = None;

/// HttpResponse is a tuple containing two primary pieces
/// of information. The first parameter is the parsed header
/// and the third parameter is the parsed content.
pub type HttpResponse = (Option<Str>, Option<Str>);

enum ParserState {
    LookForStart,
    LookForContent,
    ReadContent,
    Done,
}

fn init_buffer() {
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
    }
}

pub fn parse_http_request(rx_buffer: &Str, header: &mut Str, content: &mut Str) -> bool {
    // Ensure buffers are setup
    init_buffer();
    header.clear();
    content.clear();
    
    let buffer = unsafe { BUFFER.as_mut().unwrap() };
    let line = unsafe { LINE.as_mut().unwrap() };
    let mut content = Str::new();
    let mut header = Str::new();

    let mut ipd = str!(b"+IPD,");
    let mut colon = str!(b":");
    let mut crnl = str!(b"\r\n");
    

    let mut content_length: Option<u64> = None;
    let mut state = ParserState::LookForStart;
    let mut bytes_read = 0;

    for char in rx_buffer.into_iter() {
        line.append(&[char]);
        // Read the line
        if char == b'\n' {
            // Process this content.
            match state {
                ParserState::LookForStart => {
                    // Check for +IPD,
                    if line.contains(&ipd) && line.contains(&colon) {
                        // Yay we can find out the content-length
                        let content_begin = line.index_of(&colon).unwrap();
                        content_length = Some(atoi(line.slice(ipd.len() + 2, content_begin)));
                        header.join(&line.slice(content_begin, line.len()));
                        state = ParserState::LookForContent;
                        bytes_read += header.len();
                    }

                },
                ParserState::LookForContent => {
                    bytes_read += line.len();
                    if line.contains(&crnl) && line.len() == 2 {
                        state = ParserState::ReadContent;
                    } else {
                        header.join(line);
                    }
                },
                ParserState::ReadContent => {
                    if bytes_read as u64 + line.len() as u64 > content_length.unwrap() {
                        content.join(&line.slice(0, content_length.unwrap() as usize - bytes_read + 1 ));
                    } else {
                        content.join(&line);
                    }
                    bytes_read += line.len();
                },
                _ => {

                }
            }

            line.clear();
        }
    }


    // Drop constants here.
    // NOTE: would be sweet to not have any constants...
    ipd.drop();
    colon.drop();
    crnl.drop();

    // Checksum validation
    match content_length {
        None => {
            
            header.clear();
            content.clear();

            return false;
        },
        Some(len) => {
            if bytes_read as u64 >= len - 1 {
                return true;
            } else {
                header.clear();
                content.clear();
                return false;
            }
        },
    };
}