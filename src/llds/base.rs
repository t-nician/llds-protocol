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
        Packet {
            version: CURRENT_PACKET_VERSION,
            channel: channel,
            id: id,

            header: [0u8; 5],
            payload: [0u8; 506],

            checksum: 0,

            buffer: [0u8; 512]
        }
    }


    pub fn from_buffer(buffer: &[u8; 512]) -> Packet {
        let mut packet = Packet::new(0, 0);
        
        let mut u8_buffer = [0u8; 1];
        let mut u16_buffer = [0u8; 2];

        // u8 headers

        u8_buffer[0] = buffer[0];
        packet.header[0] = u8_buffer[0];
        packet.version = u8::from_be_bytes(u8_buffer);

        u8_buffer[0] = buffer[1];
        packet.header[1] = u8_buffer[0];
        packet.channel = u8::from_be_bytes(u8_buffer);

        u8_buffer[0] = buffer[2];
        packet.header[2] = u8_buffer[0];
        packet.id = u8::from_be_bytes(u8_buffer);

        // u16 header

        u16_buffer[0] = buffer[3];
        packet.header[3] = u16_buffer[0];
        u16_buffer[1] = buffer[4];
        packet.header[4] = u16_buffer[1];

        packet.checksum = u16::from_be_bytes(u16_buffer);

        let mut buffer_index = 0;

        buffer.map(|byte| {
            packet.buffer[buffer_index] = byte;
            buffer_index += 1;
        });

        return packet;
    }


    pub fn generate_checksum(&self) -> u16 {
        let mut checksum = fletcher::Fletcher16::new();

        checksum.update(&self.payload);

        return checksum.value();
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