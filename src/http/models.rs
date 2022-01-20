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