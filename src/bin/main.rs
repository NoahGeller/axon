use std::env;
use std::collections::HashMap;

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

    let routes = HashMap::from([
        ("/".to_string(), "/index.html".to_string()),
        ("/about".to_string(), "/about.html".to_string()),
        ("/contact".to_string(), "/contact.html".to_string()),
        ("/resume".to_string(), "/resume.html".to_string()),
    ]);

    println!("Welcome to axon! Spinning up your server...");

    let mut server = Server::new(port, "./root", routes);
    println!("Server created successfully!");

    server.listen();
}
