use llds::event::*;
use llds::packet::*;

fn main() {
    let mut packet = Packet::new();
    let mut buffer: Vec<u8> = Vec::new();
    
    packet.write_string(&"Hello there!");
    packet.write_packet_to_buffer(&mut buffer);
    
    println!("{:?}", buffer);
}