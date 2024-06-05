pub struct Packet {
    version: u8,
    channel: u8,
    id: u8,

    checksum: u32,

    payload: [u8; 505],
    header: [u8; 7],
}

impl Packet {
    pub fn new(version: u8, channel: u8, id: u8, payload: [u8; 505], checksum: u32) -> Packet {
        let mut packet = Packet {
            version: version,
            channel: channel,
            id: id,

            checksum: checksum,
            payload: payload,
            header: [0_u8; 7],
        };

        packet.validate_checksum();

        return packet;
    }

    fn load_headers(&mut self) {
        let mut header_index = 0;

        //self.id.to_ne_bytes().map(|byte| { self.header })
    }

    pub fn generate_checksum(&self) -> u32 {
        let mut checksum = fletcher::Fletcher32::new();

        let mut temp_header = [0_u16; 7];
        let mut temp_payload = [0_u16; 505];

        let mut header_index = 0;
        let mut payload_index = 0;
        
        temp_header.fill_with(||{
            if header_index < 7 {
                header_index += 1;
                return self.header[header_index - 1].into();
            } else {
                return self.header[header_index].into();
            }
        });

        temp_payload.fill_with(||{
            if payload_index < 505 {
                payload_index += 1;
                return self.payload[payload_index - 1].into();
            } else {
                return self.payload[payload_index].into();
            }
        });

        checksum.update(&temp_header);
        checksum.update(&temp_payload);

        return checksum.value();
    }

    fn validate_checksum(&mut self) {

    }
}