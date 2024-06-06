use std::io::Write;


const CURRENT_PACKET_VERSION: u8 = 1;
const DATA_SEPARATOR: u8 = 0x00;


pub struct Packet {
    pub version: u8,
    pub channel: u8,
    pub id: u8,

    pub header: [u8; 5],
    pub payload: [u8; 506],

    pub checksum: u16,

    pub buffer: [u8; 512],
}


impl Packet {
    pub fn new(id: u8, channel: u8) -> Packet {
        let mut packet = Packet {
            version: CURRENT_PACKET_VERSION,
            channel: channel,
            id: id,

            header: [0u8; 5],
            payload: [0u8; 506],

            checksum: 0,

            buffer: [0u8; 512]
        };

        packet.write_packet_to_buffer();

        return packet;
    }


    pub fn from_buffer(buffer: &[u8; 512]) -> Packet {
        let mut packet = Packet::new(0, 0);
        let buffer_cursor = &mut packet.buffer;

        buffer_cursor.copy_from_slice(buffer);

        packet.load_packet_from_buffer();

        return packet;
    }

    
    pub fn create_response_packet(&self) -> Packet {
        return Packet::new(
            self.id,
            self.channel
        );
    }


    pub fn generate_checksum(&self) -> u16 {
        let mut checksum = fletcher::Fletcher16::new();

        checksum.update(&self.payload);

        return checksum.value();
    }

    
    pub fn packet_checksum_valid(&self) -> bool {
        return self.checksum == self.generate_checksum();
    }


    pub fn load_packet_from_buffer(&mut self) {
        let mut u8_buffer = [0u8; 1];
        let mut u16_buffer = [0u8; 2];

        let mut buffer_u8_from_be_bytes = |buffer_index: usize| {
            u8_buffer[0] = self.buffer[buffer_index];
            return u8::from_be_bytes(u8_buffer);
        };

        let mut buffer_u16_from_be_bytes = |buffer_index: usize| {
            u16_buffer[0] = self.buffer[buffer_index];
            u16_buffer[1] = self.buffer[buffer_index + 1];
            return u16::from_be_bytes(u16_buffer);
        };

        // u8 headers
        self.version = buffer_u8_from_be_bytes(0);
        self.channel = buffer_u8_from_be_bytes(1);
        self.id = buffer_u8_from_be_bytes(2);

        // u16 header
        self.checksum = buffer_u16_from_be_bytes(3);

        let payload_len = self.payload.len();
        let payload_offset = 512 - payload_len;

        let mut payload_cursor = &mut self.payload[..];

        payload_cursor.write(&self.buffer[payload_offset..]).unwrap();

        let panic_ready = |reason: &str| {
            panic!(
                "{:?}\nPacket Version: {:?}\nPacket Channel: {:?}\nPacket Id: {:?}\nReceived Checksum: {:?}\nGenerated Checksum: {:?}\nPacket Buffer: {:?}",
                reason,
                self.version,
                self.channel,
                self.id,
                self.checksum,
                self.generate_checksum(),
                self.buffer
            )
        };

        if self.version != CURRENT_PACKET_VERSION {
            panic_ready("Wrong packet version!");
        }

        if !self.packet_checksum_valid() {
            panic_ready("Packet failed checksum!");
        }
    }


    pub fn write_string_to_payload(&mut self, string: &String) {
        self.payload.fill(0);

        let mut payload_cursor = &mut self.payload[..];

        payload_cursor.write(string.as_bytes()).unwrap();
    }


    pub fn write_packet_to_header(&mut self) {
        let checksum = self.generate_checksum();

        let mut header_cursor = &mut self.header[..];

        // NOTE this has to be done in this order.

        header_cursor.write(&self.version.to_be_bytes()).unwrap();

        header_cursor.write(&self.channel.to_be_bytes()).unwrap();

        header_cursor.write(&self.id.to_be_bytes()).unwrap();

        header_cursor.write(&checksum.to_be_bytes()).unwrap();
    }


    pub fn write_packet_to_buffer(&mut self) {
        self.write_packet_to_header();

        self.buffer.fill(0);

        let mut buffer_cursor = &mut self.buffer[..];

        // Write the header to the buffer!

        buffer_cursor.write(&self.header).unwrap();

        // Always add the separator when the header is complete.

        buffer_cursor.write(&DATA_SEPARATOR.to_be_bytes()).unwrap();

        // Now for the payload!

        buffer_cursor.write(&self.payload).unwrap();
    }
}

