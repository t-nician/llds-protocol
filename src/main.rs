mod llds;

use llds::base::{Packet};

fn main() {
    let mut test: Packet = Packet::new(
        1 as u8, // id
        1 as u8, // desi
        1 as u8, // version
        Vec::from((5318008_u32).to_ne_bytes()), // data
    );

    println!("{:?}", test.encode_for_socket());
    println!("Hello!");
}
