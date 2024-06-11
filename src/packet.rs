pub enum Type {
    Receiving,
    Status,
    Post,
    Get
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
}