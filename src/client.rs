use std::net::{IpAddr};

use crate::user::User;
use crate::commands::{send_file, help};

pub fn client() {
    let mut input = String::new();
    let local_port: u16 = 5000;
    
    println!("Client mode activated");
    
    // Creating user bound to port `local_port`
    let user = User::new(local_port);
    println!("Binded to local port {local_port}");

    loop {
        // Loop to connect to a remote IP Address/port
        crate::helper::get_input("Please enter address and port of a server to connect to", &mut input)
                            .expect("Error: Error reading client mode command");

        let mut arguments = input.split_whitespace();

        // Getting remote IP Address and port from input
        let remote_ip = arguments.next().expect("Error: Error reading the Argument");
        let remote_ip = remote_ip.parse::<IpAddr>()
                    .expect("Error: Must type a valid IPv4 Address");
        let remote_port = arguments.next().expect("Error: Error reading the Argument");
        let remote_port = remote_port.parse::<u16>()
                        .expect("Error: Error while reading port number");
        
        loop {
            let mut input = String::new();
            crate::helper::get_input("Enter a command", &mut input)
                                .expect("Error: Error while reading server command");

            arguments = input.split_whitespace();

            
            match arguments.next() {
                Some("send_file") => match send_file::send_file(&user, (remote_ip, remote_port), &mut arguments) {
                                        Ok(value) => (),
                                        Err(e) => println!("{e}"),
                                    },
                None => println!("Error: Must send a command"),
                _ => {
                    println!("Error reading command, please type in command");
                    help::help();
                }
            }
            

        }
    }
}