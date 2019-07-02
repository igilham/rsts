pub const PACKET_SIZE: usize = 188;
pub const HEADER_SIZE: usize = 4;
pub const PAYLOAD_SIZE: usize = PACKET_SIZE - HEADER_SIZE;
pub const SYNC_BYTE: u8 = 0x47;
pub const MAX_PID: u16 = 0x1FFF;
pub const NULL_PACKET_PID: u16 = MAX_PID;

pub type Packet = [u8; PACKET_SIZE];

pub fn null_packet() -> Packet {
    let mut packet: Packet = [0xff; PACKET_SIZE];
    packet[0] = SYNC_BYTE;
    packet[1] = 0x1f;
    packet[2] = 0xff;
    return packet;
}

pub fn set_transport_error(packet: &mut Packet) {
    packet[1] |= 0x80;
}

pub fn transport_error(packet: &Packet) -> bool {
    packet[1] & 0x80 != 0
}

pub fn set_unit_start(packet: &mut Packet) {
    packet[1] |= 0x40;
}

pub fn unit_start(packet: &Packet) -> bool {
    packet[1] & 0x40 != 0
}

pub fn set_transport_priority(packet: &mut Packet) {
    packet[1] |= 0x20;
}

pub fn transport_priority(packet: &Packet) -> bool {
    packet[1] & 0x20 != 0
}

// Sets the PID. Max: 8191 (0x1fff)
pub fn set_pid(packet: &mut Packet, pid: u16) {
    packet[1] = (pid >> 8) as u8 & 0x1f;
    packet[2] = (pid & 0x00ff) as u8;
}

pub fn pid(packet: &Packet) -> u16 {
    (((packet[1] & 0x1f) as u16) << 8) | packet[2] as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null_packet() {
        let packet: Packet = null_packet();
        assert_eq!(packet[0], SYNC_BYTE);
        assert_eq!(packet[1], 0x1f);
        assert_eq!(packet[2], 0xff);
        for i in 3..PACKET_SIZE {
            assert_eq!(packet[i], 0xff);
        }
    }

    #[test]
    fn test_transport_error() {
        let mut packet = null_packet();
        set_transport_error(&mut packet);
        assert_eq!(packet[1], 0x1f | 0x80);
        assert!(transport_error(&packet));
    }

    #[test]
    fn test_unit_start() {
        let mut packet = null_packet();
        set_unit_start(&mut packet);
        assert_eq!(packet[1], 0x1f | 0x40);
        assert!(unit_start(&packet));
    }

    #[test]
    fn test_transport_priority() {
        let mut packet = null_packet();
        set_transport_priority(&mut packet);
        assert_eq!(packet[1], 0x1f | 0x20);
        assert!(transport_priority(&packet));
    }

    #[test]
    fn test_pid() {
        let mut packet = null_packet();
        for p in 0..8191 {
            set_pid(&mut packet, p);
            assert_eq!(pid(&packet), p);
        }
    }
}
