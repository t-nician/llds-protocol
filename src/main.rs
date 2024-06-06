mod llds;

use llds::base::{ Packet };

fn main() {
    let mut packet = Packet::new(
        0,
        0
    );

    packet.write_string_to_payload(
        &String::from("Hello thereee!")
    );

    println!("{:?}", packet.payload);
}
