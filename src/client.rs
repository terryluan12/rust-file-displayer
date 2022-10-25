use std::net::{UdpSocket, Ipv4Addr};
use local_ip_address::local_ip;

pub fn client() {
    let mut input = String::new();
    let local_ip = local_ip().unwrap(); 
    let local_port: u16 = 5000;

    println!("Client mode activated");
    
    // Binding to port
    let socket = UdpSocket::bind((local_ip, local_port)).expect("couldn't bind to address");
    println!("Binded to local port {local_port}");

    loop {
        // Loop to connect to a remote IP Address/port
        crate::helper::get_input("Please enter address and port of a server to connect to", &mut input)
                            .expect("Error: Error reading client mode command");

        let mut arguments = input.split_whitespace();

        // Getting remote IP Address and port from input
        let remote_ip = arguments.next().expect("Error: Error reading the Argument");
        let remote_ip = remote_ip.parse::<Ipv4Addr>()
                    .expect("Error: Must type a valid IPv4 Address");
        let remote_port = arguments.next().expect("Error: Error reading the Argument");
        let remote_port = remote_port.parse::<u16>()
                        .expect("Error: Error while reading port number");
        
        loop {
            let mut input = String::new();

            crate::helper::get_input("Enter a command", &mut input)
                                .expect("Error: Error while reading server command");

            let mut arguments = input.split_whitespace();
            
            match arguments.next() {
                Some("send_file") => crate::commands::send_file(&socket, (remote_ip, remote_port), &mut arguments),
                None => println!("Error: Must send a command"),
                _ => {
                    println!("Error reading command, please type in command");
                    crate::commands::help();
                }
            }
            

        }
    }
}