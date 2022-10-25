use std::net::{UdpSocket};
use local_ip_address::local_ip;

pub struct User {
    socket: UdpSocket,

    username: Option<String>,

}

impl User {
    pub fn new(port: u16) -> User {
        let local_ip = local_ip().expect("Error getting the local IP address in `User` initialization");
        let socket = UdpSocket::bind((local_ip, port)).expect("couldn't bind to address in `User` initialization");

        User {socket: socket, username: None}
    }
}


// All setters and getters
impl User {
    pub fn set_socket(&mut self, socket: UdpSocket){
        self.socket = socket;
    }

    pub fn set_username(&mut self, username: String){
        self.username = Some(username);
    }

    pub fn get_socket(&self) -> &UdpSocket {
        return &self.socket;
    }

}