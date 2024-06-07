/*
    Client sends empty or useless data to make contact with server.
    Server responds with socket size, header size and header layout.
    
    Client sends a packet with the header layout to check with server.
    Server respond and either says valid packet or invalid packet.

    A session has been established.

    If the server requires a password, the exchange will happen to access-
    anymore of the server.

    If not, the client now has access.

    Header u8 number association:
        Version = 0
        Checksum = 1
        Channel = 2
        Id = 3
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
