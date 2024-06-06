mod llds;

use std::thread;
use std::time::Duration;


fn main() {
    println!("Starting server...");

    thread::spawn(|| {
        use llds::server::{Server, Packet};

        let mut server = Server::new(
            "127.0.0.1".to_string(),
            8000
        );

        server.on(|received_packet: &Packet, response_packet: &mut Packet| {
            println!("Received: {:?}", received_packet.header);

            response_packet.write_string_to_payload(
                &String::from("Server response!")
            );
        });

        server.start();
    });

    thread::sleep(Duration::from_secs(1));

    println!("Starting client...");

    thread::spawn(|| {
        use llds::client::{Client, Packet};

        let mut client = Client::new("127.0.0.1".to_string());
        let mut packet = Packet::new(5, 32);

        packet.write_string_to_payload(&"hello world!".to_string());
        packet.write_packet_to_buffer();

        println!("Sending packet data! {:?}", packet.header);

        let response = client.send_packet("127.0.0.1:8000".to_string(), &packet);
        
        println!("Response from server to client! {:?}", response.header);
    });

    thread::sleep(Duration::from_secs(1));

    println!("Hanging before closing.");

    thread::sleep(Duration::from_secs(5));

    println!("Goodnight!");

}
