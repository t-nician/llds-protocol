const DATA_SEPARATOR: [u8; 2] = [0xFF, 0x00];
const PACKET_VERSION: u8 = 1;


pub struct Packet {
    pub id: u8, // 1 byte
    pub desi: u8, // 1 byte
    pub version: u8, // 1 byte
    pub checksum: u16, // 2 bytes fletcher-16

    // Currently the header is 5 bytes long.
    // header + DATA_SEPARATOR + payload + DATA_SEPARATOR.
    // end all payloads with DATA_SEPARATOR to confirm data integrity.

    pub header: Vec<u8>, // packet header.
    pub payload: Vec<u8>, // packet payload.
    
    pub encoded_packet: [u8; 512], // packet header + payload.
}


impl Packet {
    pub fn new(id: u8, desi: u8, version: u8, payload: Vec<u8>, checksum: u16) -> Packet {
        let mut new_payload = Vec::new();
        let mut separator_index = 0;

        for index in 0..512 {
            if index < payload.len() - 1 {
                new_payload.push(payload[index])
            } else {
                if separator_index < DATA_SEPARATOR.len() - 1 {
                    separator_index += 1;
                    new_payload.push(DATA_SEPARATOR[separator_index - 1]);
                } else {
                    new_payload.push(DATA_SEPARATOR[separator_index]);
                }
            }
        }

        let mut packet = Packet {
            id: id,
            desi: desi,
            checksum: checksum,
            

            version: if version == 0 { PACKET_VERSION } else { version },

            header: Vec::new(),
            payload: new_payload,

            encoded_packet: [0_u8; 512]
        };

        packet.update_packet();

        return packet;
    }


    pub fn from_buffer(buffer: [u8; 512]) -> Result<Packet, &'static str> {
        let id = buffer[0];
        let desi = buffer[1];
        let version = buffer[2];

        let mut checksum = [0_u8; 2];
        let mut payload = [0; 507];

        payload.copy_from_slice(&buffer[5..512]);
        checksum.copy_from_slice(&buffer[3..5]);

        return Ok(
            Packet::new(
                id, // id
                desi, // desi
                version, // version
                Vec::from(payload), // data,
                u16::from_ne_bytes(checksum)
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
                vector, // data,
                0
            )
        );

        //return Err("Something unexpected happened.");
    }


    pub fn encode_for_socket(&mut self) -> [u8; 512] {
        self.encoded_packet.fill(0);
        self.update_packet();

        let mut header_payload_vector = Vec::new();

        header_payload_vector.append(&mut self.header);
        header_payload_vector.append(&mut Vec::from(DATA_SEPARATOR));
        header_payload_vector.append(&mut self.payload);
        header_payload_vector.append(&mut Vec::from(DATA_SEPARATOR));

        let header_payload_length = header_payload_vector.len();
        let data_separator_length = DATA_SEPARATOR.len();

        let mut header_index = 0;
        let mut separator_index = 0;

        self.encoded_packet.fill_with(
            || { 
                if header_index < header_payload_length {
                    header_index += 1;
                    return header_payload_vector[header_index - 1];
                } else {
                    if separator_index < data_separator_length {
                        separator_index += 1;
                        return DATA_SEPARATOR[separator_index - 1];
                    } else {
                        return DATA_SEPARATOR[data_separator_length - 1];
                    }
                }
            }
        );

        return self.encoded_packet;
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

        if self.checksum != 0 {
            let mut fletcher = fletcher::Fletcher16::new();

            println!("header {:?}", self.header);
            println!("payload {:?}", self.payload);

            fletcher.update(&self.header);
            fletcher.update(&self.payload);

            println!("fletched value {:?}", fletcher.value());
            
            if self.checksum != fletcher.value() {
                panic!("This packet was created from a buffer and failed it's checksum! Something is wrong!");
            }
        }

        self.update_checksum();

        //self.encoded_packet = self.header + DATA_SEPARATOR + self.payload + DATA_SEPARATOR;
    }
}