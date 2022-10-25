// use std::net::TcpStream;
use std::io::{self, Write};
// use std::env;
mod server;
mod client;
mod helper;


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
    
    // loop {


    //     print!("Please write a command: ");
    //     io::stdout().flush().unwrap();
        
    //     let mut command = String::new();
    //     io::stdin().read_line(&mut command)
    //       .expect("failed to read line");

    //     for arg in command.split_whitespace() {
    //         println!("{}", arg)
    //     }

    // }
}
