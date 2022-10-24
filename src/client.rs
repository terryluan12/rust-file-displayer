use std::io::{self, Write};
use std::net::{UdpSocket, Ipv4Addr};
use std::fs;

pub fn client() {
    let mut input = String::new();
    let host = "127.0.0.1:52000";
    println!("Client mode activated");

    loop {
        println!("Please enter the IP address and port to connect to.");
        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input)
            .expect("Error: Error reading client mode command");
        let mut arguments = input.split_whitespace();

        // Getting IP Address
        let ip = arguments.next().expect("Error: Error reading the Argument");
        let ip = ip.parse::<Ipv4Addr>()
                    .expect("Error: Error, Must type an IPv4 Address");
        println!("IP address is {ip}");

        // Getting port
        let port = arguments.next().expect("Error: Error reading the Argument");
        let port = port.parse::<u16>()
                        .expect("Error: Error while reading port number");
        
        // Binding to port
        let socket = UdpSocket::bind(host).expect("couldn't bind to address");
    
        loop {
            let mut input = String::new();
            let mut buffer = Vec::new();

            println!("Please enter a valid path to a file to send.");
            print!("> ");
            io::stdout().flush().unwrap();

            io::stdin().read_line(&mut input)
                .expect("Error: Error reading client mode command");

            buffer = match fs::read(input.trim()) {
                Ok(file) => file,
                Err(_) => {
                    println!("Please enter a valid path to a file");
                    continue;
                }
            };
            println!("Reading file: {:?}", &mut buffer);
            
            socket.send_to(&buffer, (ip, port)).expect("couldn't send data");

        }
    }
}