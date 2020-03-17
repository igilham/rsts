pub const PACKET_SIZE: usize = 188;
pub const HEADER_SIZE: usize = 4;
pub const HEADER_SIZE_AF: usize = 6;
pub const HEADER_SIZE_PCR: usize = 12;
pub const PAYLOAD_SIZE: usize = PACKET_SIZE - HEADER_SIZE;
pub const SYNC_BYTE: u8 = 0x47;
pub const MAX_PID: u16 = 0x1FFF;
pub const NULL_PACKET_PID: u16 = MAX_PID;

pub const PCR_MAX: usize = 2576980377600;	// 2^33 * 300
pub const PCR_RATE: usize = 27000000;	// Hz
pub const SECTION_MAX_SIZE: usize = 0x1000;

// not technically defined in ISO-13818, but this seems to be the industry consensus
pub const SCRAMBLING_CLEAR: u8 = 0;
pub const SCRAMBLING_EVEN: u8 = 2;
pub const SCRAMBLING_ODD: u8 = 3;

pub type Packet = [u8; PACKET_SIZE];

/// Get a new null packet
pub fn null_packet() -> Packet {
    let mut packet: Packet = [0xff; PACKET_SIZE];
    packet[0] = SYNC_BYTE;
    set_pid(&mut packet, 0x1fff);
    set_continuity_counter(&mut packet, 0);
    set_payload(&mut packet);
    return packet;
}

/// Set the transport error indicator
pub fn set_transport_error(packet: &mut Packet) {
    packet[1] |= 0x80;
}

/// Is the transport error indicator set?
pub fn has_transport_error(packet: &Packet) -> bool {
    packet[1] & 0x80 != 0
}

/// Set the unit start indicator
pub fn set_unit_start(packet: &mut Packet) {
    packet[1] |= 0x40;
}

/// Is the unit start indicator set?
pub fn has_unit_start(packet: &Packet) -> bool {
    packet[1] & 0x40 != 0
}

/// Set the transport priority indicator
pub fn set_transport_priority(packet: &mut Packet) {
    packet[1] |= 0x20;
}

/// Is the transport priority indicator set?
pub fn has_transport_priority(packet: &Packet) -> bool {
    packet[1] & 0x20 != 0
}

/// Set the payload present indicator
pub fn set_payload(packet: &mut Packet) {
    packet[3] |= 0x10;
}

/// Is the payload present indicator set?
pub fn has_payload(packet: &Packet) -> bool {
    packet[3] & 0x10 != 0
}

/// Get the payload as a slice of bytes
pub fn payload(packet: &Packet) -> &[u8] {
    &packet[4..PACKET_SIZE]
}

/// Set the PID. Max: 8191 (0x1fff)
pub fn set_pid(packet: &mut Packet, pid: u16) {
    packet[1] = (pid >> 8) as u8 & 0x1f;
    packet[2] = (pid & 0x00ff) as u8;
}

/// Get the value of the pid
pub fn pid(packet: &Packet) -> u16 {
    (((packet[1] & 0x1f) as u16) << 8) | packet[2] as u16
}

/// Set the continuity counter. Max: 15
pub fn set_continuity_counter(packet: &mut Packet, cc: u8) {
    packet[3] = cc & 0x0f;
}

/// Reset the continuity counter to zero
pub fn zero_continuity_counter(packet: &mut Packet) {
    packet[3] = packet[3] & 0xf0;
}

/// Get the value of the continuity counter
pub fn continuity_counter(packet: &Packet) -> u8 {
    packet[3] & 0x0f
}

/// Set the adaptation field length (&indicator)
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

/// Is the adaptation field indicator set?
pub fn has_adaptation_field(packet: &Packet) -> bool {
    packet[3] & 0x20 != 0
}

/// Get the adaptation field length
pub fn adaptation_field(packet: &Packet) -> u8 {
    packet[4]
}

/// Set the scrambling mode
pub fn set_scrambling(packet: &mut Packet, scrambling: u8) {
    packet[3] = (scrambling | 0xc0) << 6;
}

/// Get the scrambling mode
pub fn scrambling(packet: &Packet) -> u8 {
    (packet[3] & 0xc0) >> 6
}

/// Set the discontinuity indicator
pub fn set_discontinuity(packet: &mut Packet) {
    packet[5] |= 0x80;
}

/// Clear the discontinuity indicator
pub fn clear_discontinuity(packet: &mut Packet) {
    packet[5] &= !0x80;
}

/// Is the discontinuity indicator set?
pub fn has_discontinuity(packet: &Packet) -> bool {
    packet[5] & 0x80 != 0
}

/// Set the random access indicator
pub fn set_random_access(packet: &mut Packet) {
	packet[5] |= 0x40;
}

/// Is the random access indicator set?
pub fn has_random_access(packet: &Packet) -> bool {
	packet[5] & 0x40 != 0
}

/// Set the stream priority indicator
pub fn set_stream_priority(packet: &mut Packet) {
	packet[5] |= 0x20;
}

/// Is the stream priority indicator set?
pub fn has_stream_priority(packet: &Packet) -> bool {
	packet[5] & 0x20 != 0
}

/// Set the programme clock reference
pub fn set_pcr(packet: &mut Packet, pcr: u64) {
    packet[5] |= 0x10;
    packet[6] = (pcr >> 25) as u8 & 0xff;
    packet[7] = (pcr >> 17) as u8 & 0xff;
    packet[8] = (pcr >> 9) as u8 & 0xff;
    packet[9] = (pcr >> 1) as u8 & 0xff;
    packet[10] = 0x7e | ((pcr << 7) as u8 & 0x80);
}

/// Does the packet have a programme clock reference field?
/// Only valid if it has an adaptation field.
pub fn has_pcr(packet: &Packet) -> bool {
    packet[5] & 0x10 != 0
}

/// Get the programme clock reference
pub fn pcr(packet: &Packet) -> u64 {
    (packet[6] as u64) << 25 | (packet[7] as u64) << 17 | (packet[8] as u64) << 9 | (packet[9] as u64) << 1 | (packet[10] as u64) >> 7
}

/// Set the programme clock reference extension
pub fn set_pcr_ext(packet: &mut Packet, ext: u16) {
    packet[10] |= (ext >> 8) as u8 & 0x1;
    packet[11] = ext as u8 & 0xff;
}

/// Get the programme clock reference extension
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
        assert_eq!(pid(&packet), NULL_PACKET_PID);
        assert_eq!(continuity_counter(&packet), 0);
        let payload = payload(&packet);
        assert!(has_payload(&packet));
        for i in 0..PAYLOAD_SIZE {
            assert_eq!(payload[i], 0xff);
        }
        assert!(has_discontinuity(&packet));
        assert!(!has_adaptation_field(&packet));
        assert!(!has_adaptation_field(&packet));
    }

    #[test]
    fn test_transport_error() {
        let mut packet = null_packet();
        set_transport_error(&mut packet);
        assert!(has_transport_error(&packet));
    }

    #[test]
    fn test_unit_start() {
        let mut packet = null_packet();
        set_unit_start(&mut packet);
        assert!(has_unit_start(&packet));
    }

    #[test]
    fn test_transport_priority() {
        let mut packet = null_packet();
        set_transport_priority(&mut packet);
        assert!(has_transport_priority(&packet));
    }

    #[test]
    fn test_payload() {
        let mut packet = null_packet();
        set_payload(&mut packet);
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
    fn test_zero_continuity_counter() {
        let mut packet = null_packet();
        set_continuity_counter(&mut packet, 5);
        assert_eq!(continuity_counter(&packet), 5);
        zero_continuity_counter(&mut packet);
        assert_eq!(continuity_counter(&packet), 0);
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
