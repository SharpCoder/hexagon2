use teensycore::math::itoa;
use teensycore::system::str::*;
use teensycore::system::vector::*;

#[derive(Copy, Clone)]
pub struct HttpHeader {
    pub key: Str,
    pub value: Str,
}

pub struct HttpRequest {
    /// The verb like GET, PUT, POST
    pub method: Str,
    /// The path following the verb like /index.html
    pub request_path: Str,
    /// The host name and tld
    pub host: Str,
    /// Any headers
    pub headers: Option<Vector::<HttpHeader>>,
    /// Any request content
    pub content: Option<Str>,
}

impl HttpRequest {
    pub fn to_str(&self) -> Str {
        let mut result = Str::new();
        result.join(&self.method);
        result.append(b" ");
        result.join(&self.request_path);
        result.append(b" HTTP/1.1\n");
        result.append(b"HOST: ");
        result.join(&self.host);
        result.append(b"\n");

        // headers
        match self.headers {
            None => {},
            Some(headers) => {
                for header in headers.into_iter() {
                    result.join(&header.key);
                    result.append(b": ");
                    result.join(&header.value);
                    result.append(b"\n");
                }
            }
        }

        // Check for contenet
        match self.content {
            None => {
                result.append(b"\n");
            },
            Some(content) => {
                result.append(b"Content-Length: ");
                result.join(&itoa(content.len() as u64));
                result.append(b"\n\n");
                result.join(&content);
            }
        }

        return result;
    }

    pub fn drop(&mut self) {
        self.method.drop();
        self.request_path.drop();
        self.host.drop();
        
        match self.headers.as_mut() {
            None => {},
            Some(headers) => {
                for idx in 0 .. headers.size() {
                    let header = headers.get_mut(idx).unwrap();
                    
                    header.key.drop();
                    header.value.drop();
                }
            }
        }

        match self.content.as_mut() {
            None => {},
            Some(content) => {
                content.drop();
            }
        }
    }
}