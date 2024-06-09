use std::io::Write;

const SEPARATOR: [u8; 2] = [0, 0];
const PACKET_VERSION: u8 = 1;

const PACKET_SIZE: u16 = 4096;
const HEADER_SIZE: u16 = 7;
const SEPARATOR_SIZE: u16 = SEPARATOR.len() as u16;

const PAYLOAD_SIZE: u16 = PACKET_SIZE - HEADER_SIZE - SEPARATOR_SIZE;

pub struct Packet {
    pub id: u8,
    pub channel: u8,

    pub payload: Vec<u8>
}

impl Packet {
    pub fn new() -> Packet {
        Packet {
            id: 0,
            channel: 0,

            payload: Vec::new()
        }
    }

    pub fn from(buffer: &[u8]) -> Packet {
        let mut packet = Packet::new();

        return packet;
    }

    pub fn write_string(&mut self, string: &str) {
        if self.payload.len() + string.len() > PAYLOAD_SIZE as usize {
            panic!("Payload side has exceeded {:?} bytes!", PAYLOAD_SIZE)
        }

        for byte in string.as_bytes() {
            self.payload.push(byte.clone());
        }
    }

    pub fn write_packet_to_buffer(&self, buffer: &mut [u8]) {
        let mut cursor = &mut buffer[..];

        let _ = cursor.write(&PACKET_VERSION.to_be_bytes());
        let _ = cursor.write(&self.generate_checksum().to_be_bytes());

        println!("cursor {:?}", cursor);

        let _ = cursor.write(&self.channel.to_be_bytes());
        let _ = cursor.write(&self.id.to_be_bytes());

        let _ = cursor.write(&SEPARATOR);

        let _ = cursor.write(&self.payload);
    }

    pub fn generate_checksum(&self) -> u32 {
        let mut checksum = fletcher::Fletcher32::new();
        let mut u32_vector: Vec<u16> = Vec::new();

        for byte in &self.payload {
            u32_vector.push(byte.clone().into());
        }

        checksum.update(&u32_vector);

        return checksum.value();
    }
}