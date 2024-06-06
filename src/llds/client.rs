const SOCKET_MIN_PORT: u16 = 8000;
const SOCKET_MAX_PORT: u16 = 10000;

#[path="base.rs"]
mod base;

pub use base::Packet;

use std::net::UdpSocket;
use rand::Rng;

pub struct Client {
    sockets: Vec<UdpSocket>,
    host: String,
}


impl Client {
    pub fn new(host: String) -> Client {
        Client {
            host: host,
            sockets: Vec::new()
        }
    }

    pub fn send_packet(&mut self, target: String, packet: &Packet) -> Packet {
        //self.socket.send_to(&packet.buffer, "127.0.0.1:8000").unwrap();
        let mut response_packet = Packet::new(packet.id, packet.channel);
        let mut address = self.host.clone();

        address.push_str(":");
        address.push_str(
            &rand::thread_rng().gen_range(SOCKET_MIN_PORT..SOCKET_MAX_PORT).to_string()
        );

        let socket = UdpSocket::bind(address).unwrap();

        socket.send_to(&packet.buffer, target).unwrap();
        socket.recv_from(&mut response_packet.buffer).unwrap();

        response_packet.load_packet_from_buffer();

        return response_packet;
    }
}