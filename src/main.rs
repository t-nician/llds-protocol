mod llds;

use llds::base::{Packet};

fn main() {
    let mut homemade_packet: Packet = Packet::new(
        1, // id
        1 as u8, // desi
        1 as u8, // version
        Vec::from((5318008u32).to_ne_bytes()), // data
        0
    );

    println!("{:?}", homemade_packet.checksum.to_ne_bytes());

    let mut received_packet: Packet = Packet::from_buffer(
        homemade_packet.encode_for_socket()
    ).unwrap();

    println!("{:?}", received_packet.payload);
    println!("Hello!");
}
