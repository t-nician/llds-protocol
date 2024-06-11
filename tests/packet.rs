use strum::IntoEnumIterator;
use llds::packet::{Packet, Type};

#[test]
fn packet_to_buffer_and_back() {
    let mut sending_packet = Packet::new(Type::Post);
    let mut receiving_buffer = Vec::new();

    sending_packet.write_payload("Hello there!".as_bytes());

    receiving_buffer.resize(sending_packet.get_size(), 0);
    sending_packet.write_to_buffer(&mut receiving_buffer);

    let received_packet = Packet::from(&receiving_buffer);

    assert_eq!(sending_packet.r#type, received_packet.r#type);
    assert_eq!(sending_packet.channel, received_packet.channel);
    assert_eq!(sending_packet.id, received_packet.id);
    assert_eq!(sending_packet.payload, received_packet.payload);
}

#[test]
fn packets_made_with_type_are_assigned_type() {
    for packet_type in Type::iter() {
        assert_eq!(
            Packet::new(packet_type).r#type, 
            packet_type
        );
    }
}

#[test]
fn packet_type_enum_ints_are_aligned() {
    for packet_type in Type::iter() {
        assert_eq!(
            packet_type, 
            Type::from(packet_type as u8), 
            "Type ints are not aligned!\nType: {:?}\nNumber: {:?}\nFrom: {:?}",
            packet_type,
            packet_type as u8,
            Type::from(packet_type as u8)
        )
    }
}