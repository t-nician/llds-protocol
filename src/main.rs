/*
    cringe plan btw.
*/
//use llds;

use llds::core::{Event, Packet};

fn main() {
    /*let mut event = llds::core::Event::new();

    event.on_transform(&|packet| {
        // i edit packet before it gets sent to the event!
    });

    event.on_channel(10, &|received_packet, response_packet|{
        // i take packets with a specific channel number!
    });

    event.on_packet(&|received_packet, response_packet|{
        // i take any packets that don't match listening channels!
    });*/

    let mut event = Event::new();

    event.set_transformer(&|packet|{
        packet.data.push(69);
    });

    event.add_channel(10, &|received_packet, response_packet| {
        println!("packet received at channel {:?}", received_packet.data);
        println!("sending back data");

        for i in 30..50 {
            response_packet.data.push(i);
        }
    });

    event.add_listener(&|packet| {
        println!("packet had no channel to go too... {:?}", packet.data);
    });

    for (channel, listener) in &event.channels {
        println!("{:?}", channel);
    }

    let mut broadcast_packet = Packet::new(10);
    let response_packet = event.fire_event(
        &mut broadcast_packet
    );

    println!("event response {:?}", response_packet.data);
}
