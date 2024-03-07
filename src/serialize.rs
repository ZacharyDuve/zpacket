
use crate::ZPacket;

enum SerializerState {
    DestinationAddress,
    SenderAddress,
    DataLength,
    Data,
    CRC,
    Done
}
pub struct ZPacketSerializer {
    state: SerializerState,
    cur_data_i: usize,
    calc_crc: u8,
    packet: ZPacket,
}

impl ZPacketSerializer {
    pub fn new(packet: ZPacket) -> Self {
        ZPacketSerializer{state: SerializerState::DestinationAddress, cur_data_i: 0, calc_crc: 0, packet}
    }

    fn calculate_crc(&mut self, b: u8) {
        match self.state {
            SerializerState::DestinationAddress => self.calc_crc = b,
            _ => self.calc_crc ^= b
        }
    }
}



impl Iterator for ZPacketSerializer {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            //Serializing this packet is done so return none to indicate nothing else to write
            SerializerState::Done => None,
            //First step is to write out the destination Address
            SerializerState::DestinationAddress => {
                self.state = SerializerState::SenderAddress;
                self.calculate_crc(self.packet.d_addr);
                Some(self.packet.d_addr)
            },
            //Second step is to write the sender address
            SerializerState::SenderAddress => {
                self.state = SerializerState::DataLength;
                self.calculate_crc(self.packet.s_addr);
                Some(self.packet.s_addr)
            },
            //Third step is to send the data length
            SerializerState::DataLength => {
                if self.packet.d_len == 0 {
                    //We have no data to send therefore jump strait to writing the CRC byte
                    self.state = SerializerState::CRC
                } else {
                    //We have data to send so move to sending data
                    self.state = SerializerState::Data
                }
                self.calculate_crc(self.packet.d_len as u8);
                Some(self.packet.d_len as u8)
            },
            //We have some data to read out
            SerializerState::Data => {
                //Due to the data length check logic in the DataLength state we know that we have at least one byte
                let cur_byte = self.packet.d[self.cur_data_i];
                self.cur_data_i += 1;

                //Need to check that we have gotten to the end of the data to then move onto the CRC
                if self.cur_data_i == self.packet.d.len() {
                    self.state = SerializerState::CRC
                }
                self.calculate_crc(cur_byte);
                Some(cur_byte)
            }
            SerializerState::CRC => {
                self.state = SerializerState::Done;
                Some(self.calc_crc)
            }
        }
    }
}