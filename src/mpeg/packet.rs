pub const PACKET_SIZE: usize = 188;
pub const HEADER_SIZE: usize = 4;
pub const PAYLOAD_SIZE: usize = PACKET_SIZE - HEADER_SIZE;
pub const SYNC_BYTE: u8 = 0x47;
pub const MAX_PID: u16 = 0x1FFF;
pub const NULL_PACKET_PID: u16 = MAX_PID;

// not technically defined in ISO-13818, but this seems to be the industry consensus
pub const SCRAMBLING_CLEAR: u8 = 0;
pub const SCRAMBLING_EVEN: u8 = 2;
pub const SCRAMBLING_ODD: u8 = 3;

pub type Packet = [u8; PACKET_SIZE];

pub fn null_packet() -> Packet {
    let mut packet: Packet = [0xff; PACKET_SIZE];
    packet[0] = SYNC_BYTE;
    packet[1] = 0x1f; // first half of NULL_PACKET_PID
    return packet;
}

pub fn set_transport_error(packet: &mut Packet) {
    packet[1] |= 0x80;
}

pub fn has_transport_error(packet: &Packet) -> bool {
    packet[1] & 0x80 != 0
}

pub fn set_unit_start(packet: &mut Packet) {
    packet[1] |= 0x40;
}

pub fn has_unit_start(packet: &Packet) -> bool {
    packet[1] & 0x40 != 0
}

pub fn set_transport_priority(packet: &mut Packet) {
    packet[1] |= 0x20;
}

pub fn has_transport_priority(packet: &Packet) -> bool {
    packet[1] & 0x20 != 0
}

pub fn set_payload(packet: &mut Packet) {
    packet[3] |= 0x10;
}

pub fn has_payload(packet: &Packet) -> bool {
    packet[3] & 0x10 != 0
}

/// Sets the PID. Max: 8191 (0x1fff)
pub fn set_pid(packet: &mut Packet, pid: u16) {
    packet[1] = (pid >> 8) as u8 & 0x1f;
    packet[2] = (pid & 0x00ff) as u8;
}

pub fn pid(packet: &Packet) -> u16 {
    (((packet[1] & 0x1f) as u16) << 8) | packet[2] as u16
}

/// Set the continuity counter. Max: 15
pub fn set_continuity_counter(packet: &mut Packet, cc: u8) {
    packet[3] = cc & 0x0f;
}

pub fn continuity_counter(packet: &Packet) -> u8 {
    packet[3] & 0x0f
}

pub fn set_adaptation_field(packet: &mut Packet, length: u8) {
    packet[3] |= 0x20;
    packet[4] = length;
    if length > 0 {
        packet[5] = 0x00;
    }
    if length > 1 {
        for i in 6..PACKET_SIZE {
            packet[i] = 0xff; // stuffing
        }
    }
}

pub fn has_adaptation_field(packet: &Packet) -> bool {
    packet[3] & 0x20 != 0
}

pub fn adaptation_field(packet: &Packet) -> u8 {
    packet[4]
}

pub fn set_scrambling(packet: &mut Packet, scrambling: u8) {
    packet[3] = (scrambling | 0xc0) << 6;
}

pub fn scrambling(packet: &Packet) -> u8 {
    (packet[3] & 0xc0) >> 6
}

pub fn set_discontinuity(packet: &mut Packet) {
    packet[5] |= 0x80;
}

pub fn clear_discontinuity(packet: &mut Packet) {
    packet[5] &= !0x80;
}

pub fn has_discontinuity(packet: &Packet) -> bool {
    packet[5] & 0x80 != 0
}

pub fn set_random_access(packet: &mut Packet) {
	packet[5] |= 0x40;
}

pub fn has_random_access(packet: &Packet) -> bool {
	packet[5] & 0x40 != 0
}

pub fn set_stream_priority(packet: &mut Packet) {
	packet[5] |= 0x20;
}

pub fn has_stream_priority(packet: &Packet) -> bool {
	packet[5] & 0x20 != 0
}

pub fn set_pcr(packet: &mut Packet, pcr: u64) {
    packet[5] |= 0x10;
    packet[6] = (pcr >> 25) as u8 & 0xff;
    packet[7] = (pcr >> 17) as u8 & 0xff;
    packet[8] = (pcr >> 9) as u8 & 0xff;
    packet[9] = (pcr >> 1) as u8 & 0xff;
    packet[10] = 0x7e | ((pcr << 7) as u8 & 0x80);
}

pub fn has_pcr(packet: &Packet) -> bool {
    packet[5] & 0x10 != 0
}

pub fn pcr(packet: &Packet) -> u64 {
    (packet[6] as u64) << 25 | (packet[7] as u64) << 17 | (packet[8] as u64) << 9 | (packet[9] as u64) << 1 | (packet[10] as u64) >> 7
}

pub fn set_pcr_ext(packet: &mut Packet, ext: u16) {
    packet[10] |= (ext >> 8) as u8 & 0x1;
    packet[11] = ext as u8 & 0xff;
}

pub fn pcr_ext(packet: &Packet) -> u16 {
    (((packet[10] as u16) << 8) & 1) | packet[11] as u16
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
        assert!(has_transport_error(&packet));
    }

    #[test]
    fn test_unit_start() {
        let mut packet = null_packet();
        set_unit_start(&mut packet);
        assert_eq!(packet[1], 0x1f | 0x40);
        assert!(has_unit_start(&packet));
    }

    #[test]
    fn test_transport_priority() {
        let mut packet = null_packet();
        set_transport_priority(&mut packet);
        assert_eq!(packet[1], 0x1f | 0x20);
        assert!(has_transport_priority(&packet));
    }

    #[test]
    fn test_payload() {
        let mut packet = null_packet();
        set_payload(&mut packet);
        assert_eq!(packet[3], 0xff | 0x10);
        assert!(has_payload(&packet));
    }

    #[test]
    fn test_pid() {
        let mut packet = null_packet();
        for p in 0..8191 {
            set_pid(&mut packet, p);
            assert_eq!(pid(&packet), p);
        }
    }

    #[test]
    fn test_continuity_counter() {
        let mut packet = null_packet();
        for cc in 0..15 {
            set_continuity_counter(&mut packet, cc);
            assert_eq!(continuity_counter(&packet), cc);
        }
    }

    #[test]
    fn test_adaptation_field_0() {
        let mut packet = null_packet();
        let length: u8 = 0;
        set_adaptation_field(&mut packet, length);
        assert!(has_adaptation_field(&packet));
        assert_eq!(adaptation_field(&packet), length);
        assert_eq!(packet[5], 0xff);
    }

    #[test]
    fn test_adaptation_field_1() {
        let mut packet = null_packet();
        let length: u8 = 1;
        set_adaptation_field(&mut packet, length);
        assert!(has_adaptation_field(&packet));
        assert_eq!(adaptation_field(&packet), length);
        assert_eq!(packet[5], 0x00);
    }

    #[test]
    fn test_adaptation_field_2() {
        let mut packet = null_packet();
        let length: u8 = 2;
        set_adaptation_field(&mut packet, length);
        assert!(has_adaptation_field(&packet));
        assert_eq!(adaptation_field(&packet), length);
        assert_eq!(packet[5], 0x00);
        for i in 6..PACKET_SIZE {
            assert_eq!(packet[i], 0xff);
        }
    }

    #[test]
    fn test_scrambling_clear() {
        let mut packet = null_packet();
        set_scrambling(&mut packet, SCRAMBLING_CLEAR);
        assert_eq!(scrambling(&packet), SCRAMBLING_CLEAR);
    }

    #[test]
    fn test_scrambling_even() {
        let mut packet = null_packet();
        set_scrambling(&mut packet, SCRAMBLING_EVEN);
        assert_eq!(scrambling(&packet), SCRAMBLING_EVEN);
    }

    #[test]
    fn test_scrambling_odd() {
        let mut packet = null_packet();
        set_scrambling(&mut packet, SCRAMBLING_ODD);
        assert_eq!(scrambling(&packet), SCRAMBLING_ODD);
    }

    #[test]
    fn test_discontinuity() {
        let mut packet = null_packet();
        set_discontinuity(&mut packet);
        assert!(has_discontinuity(&packet));
        clear_discontinuity(&mut packet);
        assert!(!has_discontinuity(&packet));
    }

    #[test]
    fn test_random_access() {
        let mut packet = null_packet();
        set_random_access(&mut packet);
        assert!(has_random_access(&packet));
    }

    #[test]
    fn test_stream_priority() {
        let mut packet = null_packet();
        set_stream_priority(&mut packet);
        assert!(has_stream_priority(&packet));
    }

    #[test]
    fn test_pcr() {
        let mut packet = null_packet();
        let p: u64 = 23647;
        set_pcr(&mut packet, p);
        assert!(has_pcr(&packet));
        assert_eq!(pcr(&packet), p);
    }

    #[test]
    fn test_pcr_ext() {
        let mut packet = null_packet();
        let ext: u16 = 137;
        set_pcr_ext(&mut packet, ext);
        assert_eq!(pcr_ext(&packet), ext);
    }
}
