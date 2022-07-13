use std::env;
use axon::http::server::Server;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        panic!("Too many arguments!");
    }

    // Default port is 8080.
    let port: u16 = match args.len() {
        n if n == 2 => args[1].parse().unwrap(),
        _ => 8080
    };

    println!("Welcome to axon! Spinning up your server...");
    let mut server = Server::new(port, "./root");
    println!("Server created successfully!");
    server.listen();
}
