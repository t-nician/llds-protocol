const DATA_SEPARATOR: [u64; 2] = [0x00, 0x00];
const PACKET_VERSION: u8 = 1;


pub struct Packet {
    id: u8, // 1 byte
    desi: u8, // 1 byte
    version: u8, // 1 byte
    checksum: u16, // 2 bytes fletcher-16

    // Currently the header is 5 bytes long.
    // header + DATA_SEPARATOR + payload + DATA_SEPARATOR.
    // end all payloads with DATA_SEPARATOR to confirm data integrity.

    encoded_header: Vec<u8>, // packet header.
    encoded_payload: Vec<u8>, // packet payload.
    
    encoded_packet: Vec<u8>, // packet header + payload.
}


impl Packet {
    pub fn new(id: u8, desi: u8, version: u8, checksum: u16) -> Packet {
        Packet {
            id: id,
            desi: desi,
            checksum: checksum,

            version: if version == 0 { PACKET_VERSION } else { version },

            encoded_header: Vec::new(),
            encoded_payload: Vec::new(),

            encoded_packet: Vec::new()
        }
    }


    pub fn from_buffer(data: [u8; 512]) -> Result<Packet, &'static str> {
        Some(
            Packet::new(
                1 as u8, // id
                1 as u8, // desi
                1 as u8, // version
                1 as u16, // checksum
            )
        );

        return Err("Something unexpected happened.");
    }


    pub fn from_vector(data: Vec<u8>) -> Result<Packet, &'static str> {
        Some(
            Packet::new(
                1 as u8, // id
                1 as u8, // desi
                1 as u8, // version
                1 as u16, // checksum
            )
        );

        return Err("Something unexpected happened.");
    }


    pub fn encode(&mut self) -> &Vec<u8> {
        
        return &self.encoded_packet;
    }
}