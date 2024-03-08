use zpacket::{deserialize::ZPacketDeserializer, ZPacket};



#[test]
fn example_no_data() {
    let dest_addr: u8 = 0x00;
    let sender_addr: u8 = 0x30;
    let data = [0u8; 0];
    let packet_to_send = ZPacket::new(dest_addr, sender_addr, &data).expect("Error creating packet to send");

    let packet_serializer = packet_to_send.to_iter();
    let mut buffer_i: usize = 0;
    let mut buffer = [0u8; 16];
    let mut packet_deserializer = ZPacketDeserializer::new();

    let mut received_packet: Option<ZPacket> = None;

    for b in packet_serializer {
        if buffer_i == buffer.len() {
            match packet_deserializer.read(&buffer) {
                (_, Ok(packet_created)) => {
                    if let Some(p) = packet_created {
                        received_packet = Some(p);
                        break;
                    }
                },
                (_, Err(_e)) => {
                    panic!("Error received creating packet");
                }
            }
        }

        buffer[buffer_i] = b;
        buffer_i += 1;
    }

    match packet_deserializer.read(&buffer[..buffer_i]) {
        (_, Ok(packet_created)) => {
            if let Some(p) = packet_created {
                received_packet = Some(p);
            }
        },
        (_, Err(_e)) => {
            panic!("Error received creating packet");
        }
    }

    if let None = received_packet {
        panic!("Expected to receive packet but didn't get anything")
    } else {
        let r_pack = received_packet.unwrap();

        assert_eq!(r_pack.dest_address(), dest_addr);
        assert_eq!(r_pack.sender_addr(), sender_addr);
        assert_eq!(r_pack.data(), data);
    }

}