mod llds;

use llds::base::Packet;
use llds::server::Server;

use std::thread;
use std::time::Duration;


fn main() {
    println!("Starting server...");

    thread::spawn(|| {
        let mut server = Server::new(
            "127.0.0.1".to_string(),
            8000
        );

        server.on(|received_packet, response_packet| {
            println!("Received: {:?}", received_packet.payload);

            response_packet.write_string_to_payload(
                &String::from("Server response!")
            );
        });

        server.start();
    });

    thread::sleep(Duration::from_secs(2));


    println!("Starting client...");

    thread::spawn(|| {
        
    });

    println!("Hanging before closing.");

    thread::sleep(Duration::from_secs(5));

    println!("Goodnight!");

}
