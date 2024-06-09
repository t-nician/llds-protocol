use llds::packet::*;
use llds::event::*;


fn main() {
    let mut event = Event::new();

    event.post(1, &|packet| {
        println!("Post wooooh!")
    });

    event.get(1, &|packet| {
        let mut packet = Packet::new();
        
        println!("Get woooooooooh!");

        packet.write_string("Wooooooooh!");

        return packet;
    });

    event.emit(Emit::Post, 1, &mut Packet::new());

    let result = event.emit(Emit::Get, 1, &mut Packet::new());

    println!("result {:?}", result.unwrap().payload);
}
