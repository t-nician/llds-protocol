mod llds;

use llds::base::{ Packet };
use llds::server::{ Server };

use std::thread;
use std::time::Duration;


fn main() {
    println!("Starting server...");

    thread::spawn(|| {
        let server = Server::new(
            "127.0.0.1".to_string(),
            8000
        );

        

        server.start();
    });

    thread::sleep(Duration::from_secs(2));


    println!("Starting client...");

    thread::spawn(|| {

    });

    /*let mut packet = Packet::new(
        155,
        23
    );

    packet.write_string_to_payload(
        &String::from("Hello theree!")
    );

    packet.write_packet_to_buffer();

    let packet_from_buffer = Packet::from_buffer(&packet.buffer);

    println!("{:?}", packet_from_buffer.payload);*/
}
