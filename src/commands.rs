use std::net::{UdpSocket, Ipv4Addr};
use std::str::SplitWhitespace;

use std::fs;


pub fn send_file(socket: &UdpSocket, addr: (Ipv4Addr, u16),  arguments: &mut SplitWhitespace) {
    let path = arguments.next().expect("Error: Missing argument after command");


    let mut buffer = match fs::read(path) {
    Ok(file) => file,
    Err(_) => {
        println!("Please enter a valid path to a file");
        return;
    }
    };
    println!("Reading file: {:?}", &mut buffer);

    socket.send_to(&buffer, addr).expect("couldn't send data");
}

pub fn help() {

}