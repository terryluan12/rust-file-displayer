use std::net::{IpAddr};
use std::str::SplitWhitespace;
use std::fs;
use crate::user::User;


pub fn send_file(user: &User, addr: (IpAddr, u16),  arguments: &mut SplitWhitespace) {
    let path = arguments.next().expect("Error: Missing argument after command");


    let mut buffer = match fs::read(path) {
    Ok(file) => file,
    Err(_) => {
        println!("Please enter a valid path to a file");
        return;
    }
    };
    println!("Reading file: {:?}", &mut buffer);

    user.get_socket().send_to(&buffer, addr).expect("couldn't send data");
}

pub fn help() {

}