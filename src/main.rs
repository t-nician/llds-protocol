mod llds;

use llds::base::{Packet};

fn main() {
    let mut homemade_packet: Packet = Packet::new(
        1 as u8, // id
        1 as u8, // desi
        1 as u8, // version
        Vec::from((5318008_u32).to_ne_bytes()), // data
        0
    );

    let mut received_packet: Packet = Packet::from_buffer(
        homemade_packet.encode_for_socket()
    ).unwrap();

    println!("{:?}", received_packet.payload);
    println!("Hello!");
}
