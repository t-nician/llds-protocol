#[path="base.rs"]
mod base;

use base::{ Packet };
use std::net::{ UdpSocket };


pub struct Server {
    socket: UdpSocket,
    host: String,
    port: u16,
}


impl Server {
    pub fn new(host: String, port: u16) -> Server {
        let mut address = host.clone();

        address.push_str(":");
        address.push_str(&port.to_string() as &str);

        Server {
            host: host,
            port: port,
            socket: UdpSocket::bind(address).unwrap(),
        }
    }


    pub fn start(&self) {
        loop {
            let mut recv_packet = Packet::new(0, 0);
            let mut resp_packet = Packet::new(0, 0);

            let (_, client_address) = self.socket.recv_from(&mut recv_packet.buffer).unwrap();

            recv_packet.load_packet_from_buffer();

            // TODO ready packet payload, run func, pass resp_packet buffer or whole packet.

            resp_packet.write_packet_to_buffer();
            
            self.socket.send_to(&mut resp_packet.buffer, client_address).unwrap();
        }
    }
}