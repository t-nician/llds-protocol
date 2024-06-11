use std::io::Write;

const PACKET_VERSION: u8 = 1;

const HEADER_SIZE: usize = 8;
const PAYLOAD_SIZE: usize = 4096 - HEADER_SIZE;

pub enum Designation {
    None = 0,
    Post = 1,
    Get = 2
}

impl Designation {
    pub fn to_u8(&self) -> u8 {
        match self {
            Designation::Get => { return 2u8; }
            Designation::Post => { return 1u8; }
            Designation::None => { return 0u8; }
        }
    }

    pub fn from_u8(u8: &u8) -> Self {
        match u8 {
            2u8 => { Designation::Get }
            1u8 => { Designation::Post }
            0u8 => { Designation::None },
            _ => { Designation::None }
        }
    }
}

pub struct Packet {
    pub id: u8,
    pub channel: u8,
    pub designation: Designation,

    pub payload: Vec<u8>
}

impl Packet {
    pub fn new(designation: Designation) -> Packet {
        Packet {
            id: 0,
            channel: 0,
            designation: designation,

            payload: Vec::new()
        }
    }

    pub fn from(buffer: &[u8]) -> Packet {
        let mut packet = Packet::new(Designation::None);

        // version + checksum + designation + channel + id + payload

        packet.designation = Designation::from_u8(&buffer[5]);
        packet.channel = buffer[6].clone();
        packet.id = buffer[7].clone();

        packet.payload = buffer[8..].to_vec();

        let packet_version = buffer[0];
        let packet_checksum = &buffer[1..5];

        if packet.generate_checksum().to_be_bytes() != packet_checksum {
            panic!(
                "Packet::from\nPacket failed checksum!\nReceived: {:?}\nGenerated: {:?}",
                packet_checksum,
                packet.generate_checksum().to_be_bytes()
            )
        }

        if packet_version != PACKET_VERSION {
            panic!(
                "Packet::from\nPacket does match the version this client/server is running on!\nReceived: {:?}\nRunning on: {:?}",
                packet_version,
                PACKET_VERSION
            )
        }

        return packet;
    }

    pub fn get_packet_size(&self) -> usize {
        return self.payload.len() + HEADER_SIZE
    }

    pub fn generate_checksum(&self) -> u32 {
        let mut checksum = fletcher::Fletcher32::new();
        let mut appropriated_vector = Vec::new();

        for byte in &self.payload {
            appropriated_vector.push(byte.clone() as u16);
        }

        checksum.update(&appropriated_vector);

        return checksum.value();
    }

    pub fn write_string(&mut self, string: &str) {
        let payload_len = self.payload.len();

        if payload_len + string.len() > PAYLOAD_SIZE {
            panic!(
                "Packet.write_string\nPayload has exceeded {:?} bytes.\nPayload size: {:?}\nTarget size: {:?}",
                PAYLOAD_SIZE,
                payload_len,
                payload_len + string.len()
            )
        }
        
        for byte in string.as_bytes() {
            self.payload.push(byte.clone());
        }
    }

    pub fn write_packet_to_buffer(&self, buffer: &mut [u8]) {
        if buffer.len() < self.get_packet_size() {
            panic!(
                "Packet.write_packet_to_buffer\nBuffer is not big enough to fit packet!\nBuffer size: {:?}\nPacket size: {:?}",
                buffer.len(),
                self.get_packet_size()
            )
        }

        let mut cursor = &mut buffer[..];

        cursor.write(&PACKET_VERSION.to_be_bytes()).unwrap();
        
        cursor.write(&self.generate_checksum().to_be_bytes()).unwrap();

        cursor.write(&self.designation.to_u8().to_be_bytes()).unwrap();

        cursor.write(&self.channel.to_be_bytes()).unwrap();
        cursor.write(&self.id.to_be_bytes()).unwrap();

        cursor.write(&self.payload).unwrap();
    }
}