//! Types associated with HTTP requests.

use std::fmt;
use std::str;

/// HTTP methods that can be processed by the server.
#[derive(Copy, Clone, PartialEq)]
pub enum Method {
    Get,
    Head,
    NotImplemented,
}

/// Data associated with a single HTTP request.
pub struct Request {
    method: Method,
    uri: String,
}

impl Request {
    /// Prepares a new HTTP request.
    pub fn new(buf: &[u8]) -> Request {
        let raw_request = str::from_utf8(&buf).unwrap();
        let sections: Vec<&str> = raw_request.splitn(2, "\r\n").collect();

        let start_line: Vec<&str> = sections[0].split(' ').collect();

        let method = match start_line[0] {
            "GET" => Method::Get,
            "HEAD" => Method::Head,
            _ => Method::NotImplemented,
        };

        let uri = start_line[1].to_string();

        Request { method, uri }
    }
    /// Returns the HTTP method of the request.
    pub fn get_method(&self) -> Method {
        self.method
    }
    /// Returns the URI being requested.
    pub fn get_uri(&self) -> &str {
        &self.uri
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let method_str = match self.method {
            Method::Get => "GET",
            Method::Head => "HEAD",
            Method::NotImplemented => "UNKNOWN",
        };
        write!(f, "{} {}", method_str, &self.uri)
    }
}
