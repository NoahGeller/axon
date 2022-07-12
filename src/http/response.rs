//! Types associated with HTTP responses.

use std::fmt;

use crate::http::request::Method;

/// HTTP response statuses made available by the Server.
pub enum Status {
    Ok,
    NotFound,
    InternalError,
    NotImplemented,
}

impl Status {
    /// Returns the status code corresponding to the Status.
    pub fn code(&self) -> u16 {
        match self {
            Status::Ok => 200,
            Status::NotFound => 404,
            Status::InternalError => 500,
            Status::NotImplemented => 501,
        }
    }
    /// Returns the status text corresponding to the Status.
    pub fn text(&self) -> &str {
        match self {
            Status::Ok => "OK",
            Status::NotFound => "Not Found",
            Status::InternalError => "Internal Server Error",
            Status::NotImplemented => "Not Implemented",
        }
    }
}

/// Data associated with a single HTTP response.
pub struct Response {
    status: Status,
    headers: Vec<String>,
    body: Vec<u8>,
    request_method: Method,
}

impl Response {
    /// Prepares a new HTTP response.
    pub fn new(status: Status, headers: Vec<String>, body: Vec<u8>, request_method: Method) -> Response {
        Response {
            status,
            headers,
            body,
            request_method
        }
    }
    /// Create a byte Vec containing the Response data, suitable for writing to a TCP stream.
    pub fn render(&mut self) -> Vec<u8> {
        let status_line = format!("HTTP/1.1 {} {}", self.status.code(), self.status.text());

        let headers = self.headers.join("\r\n");

        let pre_body = format!(
            "{}\r\n{}\r\n\r\n",
            status_line,
            headers
        );

        let mut response = pre_body.into_bytes();
        if self.request_method != Method::Head {
            response.append(&mut self.body);
        }
        response
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.status.code(), self.status.text())
    }
}
