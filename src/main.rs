/*
    cringe plan btw.
*/
//use llds;

fn main() {
    let mut event = llds::core::Event::new();

    event.on_transform(&|packet| {
        // i edit packet before it gets sent to the event!
    });

    event.on_channel(10, &|received_packet, response_packet|{
        // i take packets with a specific channel number!
    });

    event.on_packet(&|received_packet, response_packet|{
        // i take any packets that don't match listening channels!
    });
}
