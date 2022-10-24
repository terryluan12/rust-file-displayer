use std::io::{self, Write};
use std::net::{UdpSocket, Ipv4Addr};

pub fn client() {
    let mut input = String::new();
    let mut buffer = [1; 10];
    let host = "127.0.0.1:52000";
    println!("Client mode activated");
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input)
            .expect("Error: Error reading client mode command");
        let mut arguments = input.split_whitespace();

        let ip = arguments.next().expect("Error: Error reading the Argument");
        let ip = ip.parse::<Ipv4Addr>()
                    .expect("Error: Error, Must type an IPv4 Address");
        println!("IP address is {ip}");

        let port = arguments.next().expect("Error: Error reading the Argument");
        let port = port.parse::<u16>()
                        .expect("Error: Error while reading port number");
        
        let socket = UdpSocket::bind(host).expect("couldn't bind to address");
        socket.send_to(&buffer, (ip, port)).expect("couldn't send data");
        println!("Sent data {:?}", buffer);


        
    }
}