const DATA_SEPARATOR: [u64; 2] = [0x00, 0x00];


pub struct Packet {
    id: u8,
    desi: u8,
    version: u8,
    checksum: u8,

    encoded_headers: [u8; 4], // u8 takes one byte, 4 headers
    encoded_payload: Vec<u8>, // packet payload ready for sending
    encoded_packet: Vec<u8>, // header + body data
}


impl Packet {
    pub fn from_vec(data: Option<Vec<u8>>) -> Result<Packet, &'static str> {


        return Err("Something unexpected happened.");
    }

}