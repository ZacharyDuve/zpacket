
//Max length is 255 because 0 takes up one possibility
const ZPACKET_DATA_LENGTH: usize = 255;
//Maximum of 64 addresses
const MAX_ADDRESS: u8 = 63;

pub enum ZPacketCreateError {
    DestinationAddressOutOfRange,
    SenderAddressOutOfRange,
    BadCRC,
}

pub struct ZPacket {
    d_addr: u8,
    s_addr: u8,
    d: [u8; ZPACKET_DATA_LENGTH],
    d_len: usize
}

impl ZPacket {
    fn new(dest_addr: u8, sender_addr: u8, data: &[u8]) -> Result<Self, ZPacketCreateError> {

        if dest_addr > MAX_ADDRESS {
            return Err(ZPacketCreateError::DestinationAddressOutOfRange);
        }
        
        if sender_addr > MAX_ADDRESS {
            return Err(ZPacketCreateError::SenderAddressOutOfRange);
        }

        let mut zp = ZPacket{d_addr: dest_addr, s_addr: sender_addr, d: [0; ZPACKET_DATA_LENGTH], d_len: data.len()};
        
        let mut i: usize = 0;

        for cur_b in data {
            zp.d[i] = *cur_b;

            i += 1;
        }

        Ok(zp)
    }

    pub fn dest_address(&self) -> u8 {
        self.d_addr
    }

    pub fn sender_addr(&self) -> u8 {
        self.s_addr
    }

    pub fn data(&self) -> &[u8] {
        &self.d[..self.d_len]
    }
}

enum ZPacketDeserializerReadState {
    ReadDestAddr,
    ReadSenderAddr,
    ReadLength,
    ReadData,
    ReadCRC,
}

// pub struct ZPacketDeserializerParseResults {
//     bytes_read: usize,
//     packet_created: Option<ZPacket>
// }

// impl ZPacketDeserializerParseResults {
//     pub fn number_bytes_read(&self) -> usize {
//         self.bytes_read
//     }

//     pub fn packet(&self) -> Option<ZPacket> {
//         self.packet_created
//     }
// }

const ADDR_MASK: u8 = 0x3F;
const ZPACKET_START_BITS: u8 = 0x80;

pub struct ZPacketDeserializer {
    read_state: ZPacketDeserializerReadState,
    p_d_addr: u8,
    p_s_addr: u8,
    p_data_len: usize,
    p_data: [u8; ZPACKET_DATA_LENGTH],
    p_data_i: usize,
    p_calc_crc: u8,
}

impl ZPacketDeserializer {
    pub fn new() -> Self {
        ZPacketDeserializer{
            read_state: ZPacketDeserializerReadState::ReadDestAddr, 
            p_d_addr: 0, 
            p_s_addr: 0,
            p_data_len: 0,
            p_data: [0u8; ZPACKET_DATA_LENGTH],
            p_data_i: 0,
            p_calc_crc: 0
        }
    }

    pub fn read(&mut self, data_in: &[u8]) -> (usize, Result<Option<ZPacket>, ZPacketCreateError>) {
        let mut num_bytes_read: usize = 0;

        for cur_b in data_in {
            num_bytes_read += 1;

            match self.read_state {
                ZPacketDeserializerReadState::ReadDestAddr => {
                    if *cur_b & ADDR_MASK == ZPACKET_START_BITS {
                        //Successfully read the start bits
                        //Read out the address
                        self.p_d_addr = *cur_b & ADDR_MASK;
                        //Need to start calculating the crc
                        self.p_calc_crc = *cur_b;
                        //Need to read the sender next
                        self.read_state = ZPacketDeserializerReadState::ReadSenderAddr;
                    }
                    //Else would be to fall back to ReadDestAddr state which is where we are
                }
                ZPacketDeserializerReadState::ReadSenderAddr => {
                    //Save the address portion as the sender
                    self.p_s_addr = *cur_b * ADDR_MASK;
                    //Currently the Most significant two bits are reserved for future use
                    //XOR the byte with the crc
                    self.p_calc_crc ^= *cur_b;

                    //Set state to read length next
                    self.read_state = ZPacketDeserializerReadState::ReadLength;
                }
                ZPacketDeserializerReadState::ReadLength => {
                    //Save the byte as the length
                    self.p_data_len = *cur_b as usize;

                    //XOR the byte with the crc
                    self.p_calc_crc ^= *cur_b;

                    //If length is 0 then skip to read crc otherwise go to read data
                    if self.p_data_len == 0 {
                        self.read_state = ZPacketDeserializerReadState::ReadCRC
                    } else {
                        self.read_state = ZPacketDeserializerReadState::ReadData
                    }
                }
                ZPacketDeserializerReadState::ReadData => {
                    self.p_data[self.p_data_i] = *cur_b;
                    self.p_data_i += 1;

                    //XOR the byte with the crc
                    self.p_calc_crc ^= *cur_b;
                    
                    //If we have read all of the bytes then go to read crc step
                    if self.p_data_i == self.p_data_len {
                        self.read_state = ZPacketDeserializerReadState::ReadCRC
                    }
                }
                ZPacketDeserializerReadState::ReadCRC => {
                    if *cur_b == self.p_calc_crc {
                        //CRC matched what we calculated therefore we have a good packet
                        let zp_res = ZPacket::new(self.p_d_addr, self.p_s_addr, &self.p_data[..self.p_data_len]);

                        match zp_res {
                            Ok(zp) => return (num_bytes_read, Ok(Some(zp))),
                            Err(e) => return (num_bytes_read, Err(e)),
                        }
                    } else {
                        //CRC failed so return bytes read and error
                        return (num_bytes_read, Err(ZPacketCreateError::BadCRC));
                    }
                }
            }
        }

        (num_bytes_read, Ok(None))
    }

    

}



// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
