pub enum Header {
    Checksum,
    Version,
    Channel,
    Size,
    Id
}

pub struct Packet {
    header: Vec<u8>,
    payload: Vec<u8>,
}

impl Packet {
    pub fn new() -> Packet {
        Packet {
            header: Vec::new(),
            payload: Vec::new()
        }
    }

    pub fn set_u8_header(&mut self, header: Header, value: &u8) {
        match header {
            Header::Version => {}
            Header::Channel => {},
            Header::Size => {}
            _ => {}
        }
    }

    pub fn set_u16_header(&mut self, header: Header, value: &u16) {
        match header {
            Header::Size => {},
            _ => {}
        }
    }

    pub fn set_u32_header(&mut self, header: Header, value: &u32) {
        match header {
            Header::Checksum => {},
            _ => {}
        }
    }
}