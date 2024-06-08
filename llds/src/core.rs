const DATA_SEPARATOR: [u8; 2] = [0, 0];

use std::io::Write;

#[derive(Debug)]
pub enum Header {
    Checksum,
    Version,
    Channel,
    Size,
    Id
}

#[derive(Debug)]
pub struct Packet {
    pub header: Vec<u8>,
    pub payload: Vec<u8>,
}

impl Packet {
    pub fn new() -> Packet {
        Packet {
            header: Vec::new(),
            payload: Vec::new()
        }
    }

    pub fn does_value_fit_header(header: &Header, value: &usize) -> bool {
        let copied_value = value.clone();
        let mut result = false;

        match header {
            Header::Version => { result = u8::try_from(copied_value).is_ok(); }
            Header::Checksum => { result = u32::try_from(copied_value).is_ok(); }
            Header::Size => { result = u16::try_from(copied_value).is_ok(); }
            Header::Channel => { result = u8::try_from(copied_value).is_ok(); }
            Header::Id => { result = u8::try_from(copied_value).is_ok(); }
            _ => {}
        }

        return result;
    }

    fn write_to_header(&mut self, starting_index: usize, buffer: &[u8]) {
        // TODO fix, this no work bc buffer is always [u8; 8].
        println!("{:?} {:?}", starting_index, buffer);

        let buffer_len = buffer.len();

        if self.header.len() < starting_index + buffer_len{
            for _ in 0..(starting_index + buffer_len) - self.header.len() {
                self.header.push(0);
            }
        }

        for offset in 0..buffer_len {
            let _ = std::mem::replace(&mut self.header[starting_index + offset], buffer[offset]);
        }
    }   

    pub fn set_header(&mut self, header: Header, value: usize) {
        if !Packet::does_value_fit_header(&header, &value) {
            panic!("Value: {:?} could not fit in {:?}", value, header);
        }
        
        match header {
            Header::Version => { 
                self.write_to_header(0, &value.to_be_bytes())
            },

            Header::Checksum => { 
                self.write_to_header(1, &value.to_be_bytes()) 
            },

            Header::Size => { 
                self.write_to_header(5, &value.to_be_bytes())
            },

            Header::Channel => { 
                self.write_to_header(7, &value.to_be_bytes())
            },

            Header::Id => { 
                self.write_to_header(8, &value.to_be_bytes())
            },
        }
    }

    pub fn write_to_buffer(&self, buffer: &mut [u8]) {
        let mut cursor = &mut buffer[..];
        cursor.write(&self.header).unwrap();
        cursor.write(&DATA_SEPARATOR).unwrap();
        cursor.write(&self.payload).unwrap();
    }
}