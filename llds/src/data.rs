use text_tables;
use std::io::Write;

const PACKET_VERSION: u8 = 1;
const HEADER_SEPARATOR: [u8; 2] = [0, 0];

const HEADER_SIZE: usize = 9;

pub struct Packet {
    pub version: u8,
    pub channel: u8,
    pub id: u8,

    pub size: u16,
    pub checksum: u16,

    pub header: Vec<u8>,
    pub payload: Vec<u8>,
}


impl Packet {
    pub fn new(id: u8, channel: u8) -> Packet {
        Packet {
            version: PACKET_VERSION,
            channel: channel,
            id: id,

            size: 0,
            checksum: 0,

            header: Vec::new(),
            payload: Vec::new(),
        }
    }

    pub fn from_buffer(buffer: &[u8]) -> Packet {
        if buffer.len() < HEADER_SIZE {
            panic!("Buffer is not big enough for a header!");
        }

        let mut u8_buffer = [0u8; 1];
        let mut u16_buffer = [0u8; 2];

        let mut get_u8_from_buffer = |index| {
            u8_buffer[0] = buffer[index];
            return u8::from_be_bytes(u8_buffer);
        };

        let mut get_u16_from_buffer = |index_a, index_b| {
            u16_buffer[0] = buffer[index_a];
            u16_buffer[1] = buffer[index_b];
            return u16::from_be_bytes(u16_buffer);
        };

        let mut packet = Packet {
            version: get_u8_from_buffer(0),
            size: get_u16_from_buffer(1, 2),
            checksum: get_u16_from_buffer(3, 4),
            channel: get_u8_from_buffer(5),
            id: get_u8_from_buffer(6),
            
            header: Vec::new(),
            payload: Vec::new()
        };

        for index in HEADER_SIZE..buffer.len() {
            if buffer[index] == 0 {
                break;
            }

            packet.payload.push(buffer[index])
        }

        packet.checksum_valid_or_panic();
        packet.update_header();

        return packet;
    }

    pub fn generate_encoded_packet_vector(&self) -> Vec<u8> {
        let mut result = self.header.clone();

        result.write(&HEADER_SEPARATOR.to_vec()).unwrap();
        result.write(&self.payload).unwrap();

        return result;
    }

    pub fn generate_checksum(&self) -> u16 {
        let mut checksum = fletcher::Fletcher16::new();
        
        checksum.update(&self.payload);

        return checksum.value();
    }

    pub fn checksum_valid_or_panic(&mut self) {
        if self.checksum != self.generate_checksum() {

            self.println();
            panic!(
                "Invalid checksum!\nGenerated: {:?}",
                self.generate_checksum()
            )
        }
    }

    pub fn update_header(&mut self) {
        self.size = self.payload.len() as u16;
        self.checksum = self.generate_checksum();

        self.header.clear();

        self.header.write(&self.version.to_be_bytes()).unwrap(); // [0]
        self.header.write(&self.size.to_be_bytes()).unwrap(); // [0, 0]
        self.header.write(&self.checksum.to_be_bytes()).unwrap(); // [0, 0]
        self.header.write(&self.channel.to_be_bytes()).unwrap(); // [0]
        self.header.write(&self.id.to_be_bytes()).unwrap(); // [0]
    }

    pub fn write_vector(&mut self, vector: &Vec<u8>) {
        self.payload.clear();
        self.payload.write(vector).unwrap();

        self.update_header();
    }

    pub fn write_string(&mut self, string: &String) {
        self.write_vector(&string.clone().into());
    }

    pub fn write_str(&mut self, str: &str) {
        self.write_vector(&str.into())
    }

    pub fn write_packet_to_buffer(&mut self, buffer: &mut [u8]) {
        let mut cursor = &mut buffer[..];

        self.update_header();

        cursor.write(&self.header).unwrap();
        cursor.write(&HEADER_SEPARATOR).unwrap();
        cursor.write(&self.payload).unwrap();
    }

    pub fn println(&mut self) {
        let keyword_ref = ["version", "size", "checksum", "channel", "id"];
        let number_ref = [
            &self.version.to_string() as &str,
            &self.size.to_string() as &str,
            &self.checksum.to_string() as &str,
            &self.channel.to_string() as &str,
            &self.id.to_string() as &str
        ];

        let mut payload = String::new();
        
        for byte in self.payload.iter() {
            payload.push_str(&byte.to_string());
            payload.push_str(" ");
        }

        payload = payload.trim_end().to_string();

        let mut buffer = Vec::new();

        text_tables::render(&mut buffer, vec![keyword_ref, number_ref]).unwrap();
        text_tables::render(&mut buffer, vec![["Payload"], [&payload]]).unwrap();

        println!("{}", std::str::from_utf8(&buffer).unwrap());
    }
}