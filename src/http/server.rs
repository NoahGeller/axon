//! HTTP server types.

use std::fs;
use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;

use crate::http::request::{Method, Request};
use crate::http::response::{Status, Response};

/// Context associated with a server listening over TCP.
pub struct Server {
    listener: TcpListener,
    root: String,
}

impl Server {
    /// Create a new HTTP server.
    pub fn new(port: u16, root: &str) -> Server {
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        let listener = TcpListener::bind(addr).unwrap();

        Server {
            listener,
            root: String::from(root),
        }
    }
    /// Listen for new connections.
    pub fn listen(&mut self) {
        for stream in self.listener.incoming() {
            let root = self.root.clone();
            thread::spawn(move || {
                handle_client(stream.unwrap(), root);
            });
        }
    }
}

/// Dispatched to handle a new connection.
fn handle_client(mut stream: TcpStream, root: String) {
    let mut buf = [0; 1024];
    stream
        .read(&mut buf[..])
        .expect("Error reading HTTP request into buffer.");

    let request = Request::new(&buf);

    let index = "/index.html";
    let uri = match request.get_uri() {
        "/" => index,
        uri => uri,
    };

    let path = format!("{}{}", root, uri);
    let mut response = match request.get_method() {
        Method::Get => {
            get(path)
        }
        Method::Head => {
            head(path)
        }
        Method::NotImplemented => not_implemented()
    };

    stream
        .write_all(&response.render())
        .expect("Error writing HTTP response to TCP stream.");
    stream
        .flush()
        .expect("Error flushing TCP stream after HTTP response.");
}

/// Carry out a GET request.
fn get(path: String) -> Response {
    let mut status = Status::Ok;

    let not_found = "<h1>File not found.</h1>";
    let internal_error = "<h1>Internal server error.</h1>";

    let body = match load(path) {
        Ok(content) => content,
        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                status = Status::NotFound;
                not_found.as_bytes().to_vec()
            } else {
                status = Status::InternalError;
                internal_error.as_bytes().to_vec()
            }
        }
    };

    let headers = vec![format!("Content-Length: {}", body.len()), "Server: Axon".to_string()];

    Response::new(status, headers, body, Method::Get)
}

/// Carry out a HEAD request.
fn head(path: String) -> Response {
    let mut status = Status::Ok;

    let not_found = "<h1>File not found.</h1>";
    let internal_error = "<h1>Internal server error.</h1>";

    let body = match load(path) {
        Ok(content) => content,
        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                status = Status::NotFound;
                not_found.as_bytes().to_vec()
            } else {
                status = Status::InternalError;
                internal_error.as_bytes().to_vec()
            }
        }
    };

    let headers = vec![format!("Content-Length: {}", body.len()), "Server: Axon".to_string()];

    Response::new(status, headers, body, Method::Head)
}

/// Respond to a request which isn't implemented.
fn not_implemented() -> Response {
    let status = Status::NotImplemented;
    let body = "<h1>Request not implemented.</h1>".as_bytes().to_vec();
    Response::new(status, Vec::new(), body, Method::NotImplemented)
}

/// Load a file's raw bytes.
fn load(path: String) -> Result<Vec<u8>, io::Error> {
    fs::read(path)
}
