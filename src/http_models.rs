/*
    This really has no place being in a kernel, but whatever.
    It's a bunch of useful structs for http stuff.
*/
use teensycore::*;
use teensycore::system::vector::*;
use teensycore::system::str::*;

pub struct HttpHeader {
    pub key: Str,
    pub value: Str,
}

pub struct HttpRequest {
    pub method: Str,
    pub request_uri: Str,
    pub host: Str,
    pub headers: Option<Vector::<HttpHeader>>,
    pub content: Option<Str>,
}

impl HttpRequest {
    pub fn as_vec(&self) -> Str {
        // Formulate the request
        let mut result = Vector::new();
        
        // Request line
        result.join(&self.method);
        result.push(b' ');
        result.join(&self.request_uri);
        result.join(&vec_str!(b" HTTP/1.1\r\n"));

        // Host line
        result.join(&vec_str!(b"Host: "));
        result.join(&self.host);
        result.push(b'\r');
        result.push(b'\n');

        // Headers
        match self.headers {
            None => {},
            Some(headers) => {
                for idx in 0 .. headers.size() {
                    let header = headers.get(idx).unwrap();
                    result.join(&header.key);
                    result.join(&vec_str!(b": "));
                    result.join(&header.value);
                    result.join(&vec_str!(b"\r\n"));
                }
            }
        }

        // Final line
        result.push(b'\r');
        result.push(b'\n');

        // Content if applicable
        match self.content {
            None => {},
            Some(content) => {
                result.join(&content);
            }
        }

        return result;
    }
}



#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn http_model_serialization() {
        let basic_request = HttpRequest {
            method: vec_str!(b"GET"),
            request_uri: vec_str!(b"/"),
            host: vec_str!(b"joshcole.dev"),
            headers: None,
            content: None,
        };

        let comparator = vec_str!(b"GET / HTTP/1.1\r\nHost: joshcole.dev\r\n\r\n");
        let serialized = basic_request.as_vec();

        assert_eq!(comparator.size(), serialized.size());
        for i in 0 .. comparator.size() {
            assert_eq!(serialized.get(i), comparator.get(i));
        }
    }
}