use std::io::{self, Write};
use std::net::{UdpSocket, Ipv4Addr};
use std::fs;
use local_ip_address::local_ip;

pub fn client() {
    let mut input = String::new();
    let localIp = local_ip().unwrap();
    let localPort: u16 = 5000;

    println!("Client mode activated");
    
    // Binding to port
    let socket = UdpSocket::bind((localIp, localPort)).expect("couldn't bind to address");
    println!("Binded to local port {localPort}");

    loop {
        // Loop to connect to a remote IP Address/port
        println!("Please enter the IP address and port to connect to.");
        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input)
            .expect("Error: Error reading client mode command");
        let mut arguments = input.split_whitespace();

        // Getting remote IP Address and port from input
        let remoteIp = arguments.next().expect("Error: Error reading the Argument");
        let remoteIp = remoteIp.parse::<Ipv4Addr>()
                    .expect("Error: Must type a valid IPv4 Address");
        let remotePort = arguments.next().expect("Error: Error reading the Argument");
        let remotePort = remotePort.parse::<u16>()
                        .expect("Error: Error while reading port number");
        
        loop {
            let mut input = String::new();
            let mut buffer = Vec::new();

            crate::helper::getInput("Please enter a valid path to send.", &mut input).expect("Error: Error when getting input");


            buffer = match fs::read(input.trim()) {
                Ok(file) => file,
                Err(_) => {
                    println!("Please enter a valid path to a file");
                    continue;
                }
            };
            println!("Reading file: {:?}", &mut buffer);
            
            socket.send_to(&buffer, (remoteIp, remotePort)).expect("couldn't send data");

        }
    }
}