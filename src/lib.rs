use serialize::ZPacketSerializer;


mod serialize;
pub mod deserialize;

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

    pub fn to_iter(self) -> ZPacketSerializer {
        ZPacketSerializer::new(self)
    }
}