
use crate::{ZPacket, ZPacketCreateError, ZPACKET_DATA_LENGTH, ADDRESS_MASK, DEST_ADDR_PACKET_START_IDENTIFER_BITS};

enum DeserializerState {
    DestinationAddress,
    SenderAddress,
    DataLength,
    Data,
    CRC,
}

pub struct ZPacketDeserializer {
    read_state: DeserializerState,
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
            read_state: DeserializerState::DestinationAddress, 
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
                DeserializerState::DestinationAddress => {
                    if *cur_b & !ADDRESS_MASK == DEST_ADDR_PACKET_START_IDENTIFER_BITS {
                        //Successfully read the start bits
                        //Read out the address
                        self.p_d_addr = *cur_b & ADDRESS_MASK;
                        //Need to start calculating the crc
                        self.p_calc_crc = *cur_b;
                        //Need to read the sender next
                        self.read_state = DeserializerState::SenderAddress;
                    }
                    //Else would be to fall back to ReadDestAddr state which is where we are
                }
                DeserializerState::SenderAddress => {
                    //Save the address portion as the sender
                    self.p_s_addr = *cur_b & ADDRESS_MASK;
                    //Currently the Most significant two bits are reserved for future use
                    //XOR the byte with the crc
                    self.p_calc_crc ^= *cur_b;

                    //Set state to read length next
                    self.read_state = DeserializerState::DataLength;
                }
                DeserializerState::DataLength => {
                    //Save the byte as the length
                    self.p_data_len = *cur_b as usize;

                    //XOR the byte with the crc
                    self.p_calc_crc ^= *cur_b;

                    //If length is 0 then skip to read crc otherwise go to read data
                    if self.p_data_len == 0 {
                        self.read_state = DeserializerState::CRC
                    } else {
                        self.read_state = DeserializerState::Data
                    }
                }
                DeserializerState::Data => {
                    self.p_data[self.p_data_i] = *cur_b;
                    self.p_data_i += 1;

                    //XOR the byte with the crc
                    self.p_calc_crc ^= *cur_b;
                    
                    //If we have read all of the bytes then go to read crc step
                    if self.p_data_i == self.p_data_len {
                        self.read_state = DeserializerState::CRC
                    }
                }
                DeserializerState::CRC => {
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