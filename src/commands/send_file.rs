use std::net::{IpAddr};
use std::str::SplitWhitespace;
use std::fs;
use std::path::{Path};

use crate::user::User;
use crate::file;


pub fn send_file(user: &User, addr: (IpAddr, u16),  arguments: &mut SplitWhitespace) -> Result<(), &'static str> {
    let path = arguments.next().expect("Error: Missing argument after command");

    let path = Path::new(path);

    let mut buffer = match fs::read(path) {
        Ok(file) => file,
        Err(_) => return Err("Invalid path to file")
    };

    let extension = match path.extension() {
        Some(ext) => match ext.to_str() {
                                Some(value) => String::from(value),
                                None => return Err("Issue with file extension"),
                            },
        None => String::new()
    };

    let file_name = match path.file_stem() {
        Some(name) => match name.to_str() {
                                Some(value) => String::from(value),
                                None => return Err("Issue with file name"),
                            },
        None => String::new()
    };

    let file = file::File::new(file_name, String::from("test"), extension, buffer);
    let file_string = serde_json::to_string(&file).unwrap();
    let file_byte = file_string.as_bytes();

    user.get_socket().send_to(file_byte, addr).expect("couldn't send data");
    println!("Sent message");

    Ok(())
}