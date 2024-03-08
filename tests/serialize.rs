

use zpacket::ZPacket;


//Simple test of serializing a packet to bytes when the packet has no bytes
#[test]
fn serialize_no_data_packet() {
    let dest_addr: u8 = 0x00;
    let sender_addr: u8 = 0x30;
    let data = [0u8; 0];
    let packet_to_send = ZPacket::new(dest_addr, sender_addr, &data).expect("Error creating packet to send");

    let packet_serializer = packet_to_send.to_iter();
    let mut buffer_i: usize = 0;
    //Note I am hardcoding the length of what I know I need. In reality your buffer could be any size
    let mut buffer = [0u8; 4];

    for b in packet_serializer {
        buffer[buffer_i] = b;
        buffer_i += 1;
    }

    //Dest address
    assert_eq!(buffer[0], 0b1000_0000 | dest_addr);
    //Sender address
    assert_eq!(buffer[1], 0b0011_1111 & sender_addr);
    //Data length
    assert_eq!(buffer[2], 0x00);
    //CRC
    assert_eq!(buffer[3], (0b1000_0000 | dest_addr) ^ (0b0011_1111 & sender_addr) ^ 0x00);
}

//Test of serializing a packet that has one byte of data
#[test]
fn serialize_one_byte_data_packet() {
    let dest_addr: u8 = 0x00;
    let sender_addr: u8 = 0x30;
    let data = [0xAAu8; 1];
    let packet_to_send = ZPacket::new(dest_addr, sender_addr, &data).expect("Error creating packet to send");

    let packet_serializer = packet_to_send.to_iter();
    let mut buffer_i: usize = 0;
    //Note I am hardcoding the length of what I know I need. In reality your buffer could be any size
    let mut buffer = [0u8; 5];

    for b in packet_serializer {
        buffer[buffer_i] = b;
        buffer_i += 1;
    }
    
    //Dest Address
    assert_eq!(buffer[0], 0b1000_0000 | dest_addr);
    let mut crc = 0b1000_0000 | dest_addr;
    //Sender Address
    assert_eq!(buffer[1], 0b0011_1111 & sender_addr);
    crc ^= 0b0011_1111 & sender_addr;
    //Data Length
    assert_eq!(buffer[2], 0x01);
    crc ^= 0x01;
    //Data
    assert_eq!(buffer[3], 0xAA);
    crc ^= 0xAA;
    //CRC
    assert_eq!(buffer[4], crc);
}

#[test]
fn serialize_full_byte_data_packet() {
    let dest_addr: u8 = 0x00;
    let sender_addr: u8 = 0x30;
    let data = [0xAAu8; zpacket::ZPACKET_DATA_LENGTH];
    let packet_to_send = ZPacket::new(dest_addr, sender_addr, &data).expect("Error creating packet to send");

    let packet_serializer = packet_to_send.to_iter();
    let mut buffer_i: usize = 0;
    //Note I am hardcoding the length of what I know I need. In reality your buffer could be any size
    let mut buffer = [0u8; zpacket::ZPACKET_DATA_LENGTH + 4];

    for b in packet_serializer {
        buffer[buffer_i] = b;
        buffer_i += 1;
    }
    
    //Dest Address
    assert_eq!(buffer[0], 0b1000_0000 | dest_addr);
    let mut crc = 0b1000_0000 | dest_addr;
    //Sender Address
    assert_eq!(buffer[1], 0b0011_1111 & sender_addr);
    crc ^= 0b0011_1111 & sender_addr;
    //Data Length
    assert_eq!(buffer[2], 0xFF);
    crc ^= buffer[2];
    //Data
    for b in &buffer[3..zpacket::ZPACKET_DATA_LENGTH+3] {
        assert_eq!(*b, 0xAA);
        crc ^= *b;
    }
    
    //CRC
    assert_eq!(buffer[zpacket::ZPACKET_DATA_LENGTH+3], crc);
}