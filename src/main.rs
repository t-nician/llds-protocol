use llds::packet::*;
use llds::event::*;


fn main() {
    let mut packet = Packet::new(Designation::Get);

    packet.channel = 2;
    packet.id = 33;

    packet.write_string("Hello world!");

    

}
