use std::net::{SocketAddr, UdpSocket};

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind udp socket");
    println!("Yo");
}
