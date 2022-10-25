use std::io::{self, Write};
use std::net::{UdpSocket, Ipv4Addr};
use std::fs::OpenOptions;
use local_ip_address::local_ip;

pub fn server() {
    let mut input = String::new();
    let mut buffer = [0; 500];
    let ip = local_ip().unwrap();
    println!("Server mode activated on {ip}");

    print!("> ");
    io::stdout().flush().unwrap();

    
    io::stdin().read_line(&mut input)
        .expect("Error: Error while reading server command");

    
    
    let port = input.trim().parse::<u16>().expect("Error: Input not an integer");
    let socket = UdpSocket::bind((ip, port)).expect("couldn't bind to address");

    println!("Listening on port {port}...");
    let (number_of_bytes, _src_address) = socket.recv_from(&mut buffer)
                                            .expect("Didn't receive data");

    let filled_buf = &mut buffer[..number_of_bytes];

    let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .open("newtest.txt")
    .expect("Error: Couldn't create a new file");

    file.write_all(&filled_buf);



}