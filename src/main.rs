mod llds;

use llds::base::{Packet};

fn main() {
    let test: Packet = Packet::new(
        1 as u8, // id
        1 as u8, // desi
        1 as u8, // version
        Vec::from((5318008_u32).to_ne_bytes()), // data
    );

    println!("{:?}", test.header);
    println!("Hello!");

    // sure it 'works' but uhh, the packet is invalid. same with how im making the one with vec.
    println!("{:?}", Packet::from_buffer([32_u8; 512]).unwrap().payload)
}
