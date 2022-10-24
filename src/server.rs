use std::io::{self, Write};
use std::net::{UdpSocket, Ipv4Addr};
pub fn server() {
    let mut input = String::new();
    let mut buffer = [0; 500];
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    println!("Server mode activated");

    print!("Please write a port number: ");
    io::stdout().flush().unwrap();
    
    io::stdin().read_line(&mut input)
        .expect("Error: Error while reading port number");
    let port = input.trim().parse::<u16>().expect("Error: Input not an integer");
    // let host = ip + ":" + &port.to_string();
    let socket = UdpSocket::bind((ip, port)).expect("couldn't bind to address");

    println!("Listening on port {port}...");
    let (number_of_bytes, _src_address) = socket.recv_from(&mut buffer)
                                            .expect("Didn't receive data");

    let filled_buf = &mut buffer[..number_of_bytes];
    println!("Got data: {:?}", filled_buf);
    

}