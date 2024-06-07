/*
    cringe plan btw.
*/
//use llds;

fn main() {
    let mut packet = llds::data::Packet::new(255, 255);
    let mut buffer = [0u8; 512];

    packet.write_str(&"Hello there");
    packet.write_packet_to_buffer(&mut buffer);

    packet.println();

    let packet_from_buffer = llds::data::Packet::from_buffer(&buffer);

    packet.println();
    
}
