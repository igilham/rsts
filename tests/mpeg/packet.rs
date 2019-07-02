extern crate rsts;

use rsts::mpeg::packet::*;

#[test]
fn test_null_packet{
    let packet: Packet = null_packet();
    assert_eq!(packet[0], SYNC_BYTE);
    assert_eq!(packet[1], 0x1f);
    assert_eq!(packet[2], 0xff);
    for i in 3..PACKET_SIZE {
        assert_eq!(packet[i], 0xff);
    }
}
