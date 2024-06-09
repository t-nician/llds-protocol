use llds::packet::*;
use llds::event::*;


fn main() {
    let mut packet = Packet::new();
    let mut buffer = Vec::new();
    
    packet.id = 35;

    packet.write_string("Hello there");

    buffer.resize(packet.get_packet_size(), 0);
    packet.write_packet_to_buffer(&mut buffer);

    println!("{:?}", Packet::from(&buffer).id);
    println!("{:?}", (33u8).to_be_bytes());
}
