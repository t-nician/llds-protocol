const CURRENT_PACKET_VERSION: u8 = 1;
const DATA_SEPARATOR: u8 = 0x00;


pub struct Packet {
    pub version: u8,
    pub channel: u8,
    pub id: u8,

    pub header: [u8; 5],
    pub payload: [u8; 506],

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

            buffer: [0u8; 512]
        }
    }

    pub fn generate_checksum(&self) -> u16 {
        let mut checksum = fletcher::Fletcher16::new();
        checksum.update(&self.payload);
        return checksum.value();
    }

    pub fn write_string_to_payload(&mut self, string: &String) {
        let payload_len = self.payload.len();
        for (index, byte) in string.bytes().enumerate() {
            if index < payload_len {
                println!("AA");
                self.payload[index] = byte;
            } else {
                panic!("YOUR STRING IS TOO BIG OMGG!");
            }
        }
    }

    pub fn write_packet_to_buffer(&mut self) {
        let mut buffer_index = 0;

        self.buffer.fill(0);

        // A better way to do this I bet. :D
        // NOTE this has to be done in this order.

        self.version.to_ne_bytes().map(|byte| { 
            buffer_index += 1; self.buffer[buffer_index - 1] = byte;
        });

        self.channel.to_ne_bytes().map(|byte| {
            buffer_index += 1; self.buffer[buffer_index - 1] = byte;
        });

        self.id.to_ne_bytes().map(|byte| {
            buffer_index += 1; self.buffer[buffer_index - 1] = byte;
        });

        self.generate_checksum().to_ne_bytes().map(|byte| {
            buffer_index += 1; self.buffer[buffer_index - 1] = byte;
        });

        // Always add the separator when the header is complete.

        DATA_SEPARATOR.to_ne_bytes().map(|byte| {
            buffer_index += 1; self.buffer[buffer_index - 1] = byte;
        });

        // Now for the payload!
        self.payload.map(|byte| {
            buffer_index += 1; self.buffer[buffer_index - 1] = byte;
        });

    }
}