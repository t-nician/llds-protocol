const DATA_SEPARATOR: [u64; 2] = [0x00, 0x00];
const PACKET_VERSION: u8 = 1;


pub struct Packet {
    id: u8,
    desi: u8,
    version: u8,
    checksum: u16, // fletcher-16

    encoded_headers: Vec<u8>, // packet headers.
    encoded_payload: Vec<u8>, // packet payload.
    encoded_packet: Vec<u8>, // packet header + payload
}


impl Packet {
    pub fn new() -> Packet {
        Packet {
            id: 0,
            desi: 0,
            version: PACKET_VERSION,
            checksum: 0,

            encoded_headers: Vec::new(),
            encoded_payload: Vec::new(),
            encoded_packet: Vec::new()
        }
    }

    
    pub fn from_buffer(data: [u8; 4096]) -> Result<Packet, &'static str> {
        Some(Packet::new());

        return Err("Something unexpected happened.");
    }


    pub fn from_vector(data: Vec<u8>) -> Result<Packet, &'static str> {
        Some(Packet::new());

        return Err("Something unexpected happened.");
    }


    pub fn encode(&mut self) {

    }
}