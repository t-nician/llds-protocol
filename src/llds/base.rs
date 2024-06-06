const CURRENT_PACKET_VERSION: u8 = 1;


pub struct Packet {
    pub checksum: u16,
    pub version: u8,
    pub channel: u8,
    pub id: u8,

    pub header: [u8; 5],
    pub payload: [u8; 507],

    pub buffer: [u8; 512],
}


impl Packet {
    pub fn new(id: u8, channel: u8) -> Packet {
        Packet {
            checksum: 0,
            version: CURRENT_PACKET_VERSION,
            channel: channel,
            id: id,

            header: [0u8; 5],
            payload: [0u8; 507],

            buffer: [0u8; 512]
        }
    }

    pub fn write_string_to_payload(&mut self, string: &String) {
        let payload_len = self.payload.len();
        for (index, byte) in string.bytes().enumerate() {
            if index < payload_len {
                self.payload[index] = byte;
            }
        }
    }
}