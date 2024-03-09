use zpacket::ZPACKET_DATA_LENGTH;


#[test]
fn deserialize_no_data_payload() {
    let dest_address: u8 = 0x03;
    let dest_data = dest_address | 0b1000_0000;
    let sender_address: u8 = 0x32;
    let sender_data = 0x3F & sender_address;
    let data_len: usize = 0;
    let crc = dest_data ^ sender_data ^ data_len as u8;

    let payload = [dest_data, sender_data, data_len as u8, crc];

    println!("{:?}", &payload);
    
    let mut packet_deserializer = zpacket::deserialize::ZPacketDeserializer::new();

    let (n_bytes_read, read_res) = packet_deserializer.read(&payload);

    assert_eq!(n_bytes_read, payload.len());

    if let Err(e) = read_res {
        panic!("Expected good read of packet but instead an error was returned, {:?}", e);
    }

    let read_packet = read_res.unwrap();

    match read_packet {
        None => panic!("Expected to get a packet from valid read but instead got nothing"),
        Some(p) => {
            assert_eq!(p.dest_address(), dest_address);
            assert_eq!(p.sender_addr(), sender_address);
            assert_eq!(p.data().len(), data_len);
        }
    }
}


#[test]
fn deserialize_max_payload_single_part() {
    let dest_address: u8 = 0x03;
    let dest_data = dest_address | 0b1000_0000;
    let sender_address: u8 = 0x32;
    let sender_data = 0x3F & sender_address;
    let data_len: usize = zpacket::ZPACKET_DATA_LENGTH;
    let mut crc = dest_data ^ sender_data ^ data_len as u8;

    let mut payload = [0x00u8; ZPACKET_DATA_LENGTH + 4];

    payload[0] = dest_data;
    payload[1] = sender_data;
    payload[2] = data_len as u8;
    let data_offset = 3;
    let junk_data: u8 = 0x55;
    for i in 0 + data_offset..ZPACKET_DATA_LENGTH + data_offset {
        payload[i] = junk_data;
        crc ^= junk_data;
    }

    payload[ZPACKET_DATA_LENGTH + 3] =  crc;

    println!("{:?}", &payload);
    
    let mut packet_deserializer = zpacket::deserialize::ZPacketDeserializer::new();

    let (n_bytes_read, read_res) = packet_deserializer.read(&payload);

    assert_eq!(n_bytes_read, payload.len());

    if let Err(e) = read_res {
        panic!("Expected good read of packet but instead an error was returned, {:?}", e);
    }

    let read_packet = read_res.unwrap();

    match read_packet {
        None => panic!("Expected to get a packet from valid read but instead got nothing"),
        Some(p) => {
            assert_eq!(p.dest_address(), dest_address);
            assert_eq!(p.sender_addr(), sender_address);
            assert_eq!(p.data().len(), data_len);

            for cur_b in p.data() {
                assert_eq!(*cur_b, junk_data);
            }
        }
    }
}

#[test]
fn deserialize_max_payload_multi_part() {
    let dest_address: u8 = 0x03;
    let dest_data = dest_address | 0b1000_0000;
    let sender_address: u8 = 0x32;
    let sender_data = 0x3F & sender_address;
    let data_len: usize = zpacket::ZPACKET_DATA_LENGTH;
    let mut crc = dest_data ^ sender_data ^ data_len as u8;

    let mut payload = [0x00u8; ZPACKET_DATA_LENGTH + 4];

    payload[0] = dest_data;
    payload[1] = sender_data;
    payload[2] = data_len as u8;
    let data_offset = 3;
    let junk_data: u8 = 0x55;
    for i in 0 + data_offset..ZPACKET_DATA_LENGTH + data_offset {
        payload[i] = junk_data;
        crc ^= junk_data;
    }

    payload[ZPACKET_DATA_LENGTH + 3] =  crc;

    println!("{:?}", &payload);
    
    let mut packet_deserializer = zpacket::deserialize::ZPacketDeserializer::new();

    let midpoint = payload.len() / 2;
    let first_part = &payload[..midpoint];
    let (first_num_bytes_read, first_read_res) = packet_deserializer.read(first_part);

    assert_eq!(first_num_bytes_read, first_part.len());
    assert!(first_read_res.is_ok());
    assert!(first_read_res.unwrap().is_none());

    let (n_bytes_read, read_res) = packet_deserializer.read(&payload[midpoint..]);

    assert_eq!(n_bytes_read, payload.len() - first_part.len());
    assert!(read_res.is_ok());
    assert!(read_res.as_ref().unwrap().is_some());

    if let Err(e) = read_res {
        panic!("Expected good read of packet but instead an error was returned, {:?}", e);
    }

    let read_packet = read_res.unwrap();

    match read_packet {
        None => panic!("Expected to get a packet from valid read but instead got nothing"),
        Some(p) => {
            assert_eq!(p.dest_address(), dest_address);
            assert_eq!(p.sender_addr(), sender_address);
            assert_eq!(p.data().len(), data_len);

            for cur_b in p.data() {
                assert_eq!(*cur_b, junk_data);
            }
        }
    }
}