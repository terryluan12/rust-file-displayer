use std::io::{Write};
use std::net::{UdpSocket};
use std::fs::OpenOptions;
use local_ip_address::local_ip;

use crate::file;

pub fn server() {
    let mut input = String::new();
    let mut buffer = [0; 500];
    let ip = local_ip().unwrap();
    println!("Server mode activated on {ip}");

    crate::helper::get_input("Enter port to connect to", &mut input)
                                .expect("Error: Error while reading server command");
    
    let port = input.trim().parse::<u16>().expect("Error: Input not an integer");
    let socket = UdpSocket::bind((ip, port)).expect("couldn't bind to address");

    println!("Listening on port {port}...");
    let (number_of_bytes, _src_address) = socket.recv_from(&mut buffer)
                                            .expect("Didn't receive data");
    let buffer_string = std::str::from_utf8(&buffer[0..number_of_bytes]).expect("Error transforming buffer to string");
    let response: file::File = serde_json::from_str(buffer_string).expect("Issue with data received");

    println!("Received a message");

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(String::from(response.get_file_name()) + "_new" + &response.get_extension())
        .expect("Error: Couldn't create a new file");

    file.write_all(&response.get_data()).expect("Error: Something went wrong writing to file");
    println!("Wrote message to file");



}