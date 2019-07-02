pub const PACKET_SIZE: usize = 188;
pub const HEADER_SIZE: usize = 4;
pub const PAYLOAD_SIZE: usize = PACKET_SIZE - HEADER_SIZE;
pub const SYNC_BYTE: u8 = 0x47;
pub const MAX_PID: u16 = 0x1FFF;

pub type Packet = [u8; PACKET_SIZE];

pub fn null_packet() -> Packet {
    let mut packet: Packet = [0xff; PACKET_SIZE];
    packet[0] = SYNC_BYTE;
    packet[1] = 0x1f;
    packet[2] = 0xff;
    return packet;
}
