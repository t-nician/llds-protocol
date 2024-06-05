mod llds;

use llds::base::{Packet};

fn main() {
    let test: Packet = Packet::new(
        1 as u8, // id
        1 as u8, // desi
        1 as u8, // version
        1 as u16, // checksum
    );
}
