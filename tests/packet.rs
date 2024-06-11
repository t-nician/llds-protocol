use strum::IntoEnumIterator;
use llds::packet::{Packet, Type};

#[test]
fn packets_made_with_type_are_assigned_type() {
    for packet_type in Type::iter() {
        assert_eq!(
            Packet::new(
                packet_type
            ).r#type, 

            packet_type
        );
    }
}




