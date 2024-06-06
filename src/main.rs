mod llds;

use llds::base::{ Packet };

fn main() {
    let mut packet = Packet::new(
        155,
        23
    );

    packet.write_string_to_payload(
        &String::from("Hello theree!")
    );

    packet.write_packet_to_buffer();

    let packet_from_buffer = Packet::from_buffer(&packet.buffer);

    println!("{:?}", packet_from_buffer.payload);
}
