use strum_macros::EnumIter; 
use std::io::Write;

const HEADER_SIZE: usize = 3;

#[derive(Debug, EnumIter, Clone, Copy, PartialEq)]
pub enum Type {
    Receiving = 0,
    Status = 1,
    Post = 2,
    Get = 3
}

impl Type {
    pub fn from(u8: u8) -> Self {
        match u8 {
            0 => { Type::Receiving }
            1 => { Type::Status }
            2 => { Type::Post }
            3 => { Type::Get }
            _ => { Type::Receiving }
        }
    }
}

pub struct Packet {
    pub r#type: Type,

    pub channel: u8,
    pub id: u8,

    pub payload: Vec<u8>
}

impl Packet {
    pub fn new(r#type: Type) -> Self {
        Packet {
            r#type: r#type,

            channel: 0,
            id: 0,

            payload: Vec::new()
        }
    }

    pub fn from(buffer: &[u8]) -> Self {
        let mut packet = Packet::new(Type::Receiving);

        packet.r#type = Type::from(buffer[0]);
        packet.channel = buffer[1];
        packet.id = buffer[2];

        packet.payload = buffer[3..].to_vec();

        return packet;
    }

    pub fn get_size(&self) -> usize {
        return self.payload.len() + HEADER_SIZE
    }

    pub fn write_payload(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.payload.push(byte.clone());
        }
    }

    pub fn write_to_buffer(&self, buffer: &mut [u8]) {
        let mut cursor = &mut buffer[..];

        cursor.write(&(self.r#type as u8).to_be_bytes()).unwrap();
        cursor.write(&self.channel.to_be_bytes()).unwrap();
        cursor.write(&self.id.to_be_bytes()).unwrap();
        cursor.write(&self.payload).unwrap();
    }
}