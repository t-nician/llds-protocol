#[path="base.rs"]
mod base;

pub use base::Packet;
use std::net::UdpSocket;


pub struct Server {
    callback: Box<dyn Fn(&Packet, &mut Packet) + 'static>,
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
            callback: Box::new(|_, _|{})
        }
    }


    pub fn on<F>(&mut self, callback: F) where F: Fn(&Packet, &mut Packet) + 'static {
        self.callback = Box::from(callback);
    }


    pub fn start(&mut self){

        loop {
            let mut recv_packet = Packet::new(0, 0);
            let mut resp_packet = Packet::new(0, 0);

            let (_, client_address) = self.socket.recv_from(&mut recv_packet.buffer).unwrap();

            recv_packet.load_packet_from_buffer();

            // TODO ready packet payload, run func, pass resp_packet buffer or whole packet.
            resp_packet.channel = recv_packet.channel;
            resp_packet.id = recv_packet.id;

            self.callback.as_ref()(&recv_packet, &mut resp_packet);

            resp_packet.write_packet_to_buffer();
            
            self.socket.send_to(&mut resp_packet.buffer, client_address).unwrap();
        }
    }
}