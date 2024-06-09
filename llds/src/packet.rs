use std::io::Write;

const SEPARATOR: [u8; 2] = [0, 0];
const PACKET_VERSION: u8 = 1;

pub const PACKET_SIZE: usize = 4096;
pub const HEADER_SIZE: usize = 7;

pub const PAYLOAD_SIZE: usize = PACKET_SIZE - HEADER_SIZE - SEPARATOR_SIZE;
pub const SEPARATOR_SIZE: usize = SEPARATOR.len();


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

        let checksum = &buffer[1..5];

        let channel = &buffer[5];
        let id = &buffer[6];

        let payload = &buffer[9..];

        packet.id = id.clone();

        packet.channel = channel.clone();
        packet.payload = payload.to_vec();

        if packet.generate_checksum().to_be_bytes() != checksum {
            panic!(
                "Packet::from(buffer &[u8]) failed checksum!\nGot: {:?}\nGenerated: {:?}",
                checksum,
                packet.generate_checksum().to_be_bytes()
            )
        }

        return packet;
    }

    pub fn get_packet_size(&self) -> usize {
        return self.payload.len() + HEADER_SIZE + SEPARATOR_SIZE;
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

    pub fn write_string(&mut self, string: &str) {
        if self.payload.len() + string.len() > PAYLOAD_SIZE {
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

        let _ = cursor.write(&self.channel.to_be_bytes());
        let _ = cursor.write(&self.id.to_be_bytes());

        let _ = cursor.write(&SEPARATOR);

        let _ = cursor.write(&self.payload);
    }
}