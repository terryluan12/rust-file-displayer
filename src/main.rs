// use std::net::TcpStream;
use std::io::{self, Write};
// use std::env;
mod server;
mod client;
mod helper;
mod commands;
mod user;

fn main() {
    // TODO implement command line arguments through std::env::args()
    print!("Select mode(server/client): ");
    io::stdout().flush().unwrap();
    let mut command = String::new();
    io::stdin().read_line(&mut command)
        .expect("Error: Error while reading mode");
    match command.trim() {
        "server" => server::server(),
        "client" => client::client(),
        _ => println!("Error: must be either `server` or client`, not {command}"),
    }
    
}
