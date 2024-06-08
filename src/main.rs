use llds::core::{Packet, Header};

fn main() {
    let mut packet = Packet::new();

    packet.set_header(Header::Version, 1);
    packet.set_header(Header::Channel, 2);
    packet.set_header(Header::Id, 3);
    packet.set_header(Header::Checksum, 35352);
    packet.set_header(Header::Size, 4096);

    println!("{:?}", packet.header);
}
