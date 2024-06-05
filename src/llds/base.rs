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

    pub header: Vec<u8>, // packet header.
    pub payload: Vec<u8>, // packet payload.
    
    encoded_packet: Vec<u8>, // packet header + payload.
}


impl Packet {
    pub fn new(id: u8, desi: u8, version: u8, payload: Vec<u8>) -> Packet {
        let mut packet = Packet {
            id: id,
            desi: desi,
            checksum: 0,
            

            version: if version == 0 { PACKET_VERSION } else { version },

            header: Vec::new(),
            payload: payload,

            encoded_packet: Vec::new()
        };

        packet.update_packet();

        return packet;
    }


    pub fn from_buffer(buffer: [u8; 512]) -> Result<Packet, &'static str> {
        return Ok(
            Packet::new(
                1 as u8, // id
                1 as u8, // desi
                1 as u8, // version
                Vec::from(buffer), // data
            )
        );

        //return Err("Something unexpected happened.");
    }


    pub fn from_vector(vector: Vec<u8>) -> Result<Packet, &'static str> {
        return Ok(
            Packet::new(
                1 as u8, // id
                1 as u8, // desi
                1 as u8, // version
                vector, // data
            )
        );

        //return Err("Something unexpected happened.");
    }


    pub fn encode(&mut self) -> &Vec<u8> {
        
        return &self.encoded_packet;
    }

    fn update_checksum(&mut self) {

        let mut new_checksum = fletcher::Fletcher16::new();

        new_checksum.update(&self.header as &[u8]);
        new_checksum.update(&self.payload as &[u8]);

        self.checksum = new_checksum.value();

        if self.header.len() != 3 {
            self.update_header();
        }

        self.checksum.to_ne_bytes().map(|byte| self.header.push(byte));
    }

    fn update_header(&mut self) {
        self.header.clear();
        
        self.id.to_ne_bytes().map(|byte| self.header.push(byte));
        self.desi.to_ne_bytes().map(|byte| self.header.push(byte));
        self.version.to_ne_bytes().map(|byte| self.header.push(byte));

    }

    fn update_packet(&mut self) {
        self.update_header();
        self.update_checksum();
    }
}