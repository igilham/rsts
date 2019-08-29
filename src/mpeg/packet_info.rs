use super::packet;

/// A higher-level representation of a transport stream packet.
#[derive(Clone, Copy)]
pub struct PacketInfo {
    packet: packet::Packet,
}

impl PacketInfo {
    pub fn null_packet() -> Self {
        PacketInfo {
            packet: packet::null_packet(),
        }
    }

    pub fn set_transport_error(&mut self) {
        packet::set_transport_error(&mut self.packet);
    }

    pub fn has_transport_error(&self) -> bool {
        packet::has_transport_error(&self.packet)
    }

    pub fn set_unit_start(&mut self) {
        packet::set_unit_start(&mut self.packet);
    }

    pub fn has_unit_start(&self) -> bool {
        packet::has_unit_start(&self.packet)
    }

    pub fn set_transport_priority(&mut self) {
        packet::set_transport_priority(&mut self.packet);
    }

    pub fn has_transport_priority(&self) -> bool {
        packet::has_transport_priority(&self.packet)
    }

    /// Set the payload present indicator
    pub fn set_payload(&mut self) {
        packet::set_payload(&mut self.packet);
    }

    /// Does the packet have a payload indicator?
    pub fn has_payload(&self) -> bool {
        packet::has_payload(&self.packet)
    }

    /// Get the payload as a slice of bytes
    pub fn payload(&self) -> &[u8] {
        packet::payload(&self.packet)
    }

    /// Sets the PID. Max: 8191 (0x1fff)
    pub fn set_pid(&mut self, pid: u16) {
        packet::set_pid(&mut self.packet, pid);
    }

    /// Get the value of the pid
    pub fn pid(&self) -> u16 {
        packet::pid(&self.packet)
    }

    /// Set the continuity counter. Max: 15
    pub fn set_continuity_counter(&mut self, cc: u8) {
        packet::set_continuity_counter(&mut self.packet, cc);
    }

    /// Reset the continuity counter to zero
    pub fn zero_continuity_counter(&mut self) {
        packet::zero_continuity_counter(&mut self.packet);
    }

    /// Get the value of the continuity counter
    pub fn continuity_counter(&self) -> u8 {
        packet::continuity_counter(&self.packet)
    }

    pub fn set_adaptation_field(&mut self, length: u8) {
        packet::set_adaptation_field(&mut self.packet, length);
    }

    pub fn has_adaptation_field(&self) -> bool {
        packet::has_adaptation_field(&self.packet)
    }

    pub fn adaptation_field(&self) -> u8 {
        packet::adaptation_field(&self.packet)
    }

    pub fn set_scrambling(&mut self, scrambling: u8) {
        packet::set_scrambling(&mut self.packet, scrambling);
    }

    pub fn scrambling(&self) -> u8 {
        packet::scrambling(&self.packet)
    }

    pub fn set_discontinuity(&mut self) {
        packet::set_discontinuity(&mut self.packet);
    }

    pub fn clear_discontinuity(&mut self) {
        packet::clear_discontinuity(&mut self.packet);
    }

    pub fn has_discontinuity(&self) -> bool {
        packet::has_discontinuity(&self.packet)
    }

    pub fn set_random_access(&mut self) {
        packet::set_random_access(&mut self.packet);
    }

    pub fn has_random_access(&self) -> bool {
        packet::has_random_access(&self.packet)
    }

    pub fn set_stream_priority(&mut self) {
        packet::set_stream_priority(&mut self.packet);
    }

    pub fn has_stream_priority(&self) -> bool {
        packet::has_stream_priority(&self.packet)
    }

    pub fn set_pcr(&mut self, pcr: u64) {
        packet::set_pcr(&mut self.packet, pcr);
    }

    pub fn has_pcr(&self) -> bool {
        packet::has_pcr(&self.packet)
    }

    pub fn pcr(&self) -> u64 {
        packet::pcr(&self.packet)
    }

    pub fn set_pcr_ext(&mut self, ext: u16) {
        packet::set_pcr_ext(&mut self.packet, ext);
    }

    pub fn pcr_ext(&self) -> u16 {
        packet::pcr_ext(&self.packet)
    }
}

impl From<packet::Packet> for PacketInfo {
    fn from(pkt: packet::Packet) -> Self {
        PacketInfo {
            packet: pkt,
        }
    }
}

impl Into<packet::Packet> for PacketInfo {
    fn into(self) -> packet::Packet {
        self.packet
    }
}

impl Default for PacketInfo {
    fn default() -> Self {
        PacketInfo::null_packet()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null_packet() {
        let p: PacketInfo = PacketInfo::null_packet();
        assert_eq!(p.pid(), packet::NULL_PACKET_PID);
        assert_eq!(p.continuity_counter(), 0);
        let payload = p.payload();
        assert!(p.has_payload());
        for i in 0..packet::PAYLOAD_SIZE {
            assert_eq!(payload[i], 0xff);
        }
        assert!(p.has_discontinuity());
        assert!(!p.has_adaptation_field());
        assert!(!p.has_adaptation_field());
    }

    #[test]
    fn test_transport_error() {
        let mut p = PacketInfo::null_packet();
        p.set_transport_error();
        assert!(p.has_transport_error());
    }

    #[test]
    fn test_unit_start() {
        let mut p = PacketInfo::null_packet();
        p.set_unit_start();
        assert!(p.has_unit_start());
    }

    #[test]
    fn test_transport_priority() {
        let mut p = PacketInfo::null_packet();
        p.set_transport_priority();
        assert!(p.has_transport_priority());
    }

    #[test]
    fn test_payload() {
        let mut p = PacketInfo::null_packet();
        p.set_payload();
        assert!(p.has_payload());
    }

    #[test]
    fn test_pid() {
        let mut p = PacketInfo::null_packet();
        for pid in 0..8191 {
            p.set_pid(pid);
            assert_eq!(p.pid(), pid);
        }
    }

    #[test]
    fn test_continuity_counter() {
        let mut p = PacketInfo::null_packet();
        for cc in 0..15 {
            p.set_continuity_counter(cc);
            assert_eq!(p.continuity_counter(), cc);
        }
    }

    #[test]
    fn test_zero_continuity_counter() {
        let mut packet = PacketInfo::null_packet();
        packet.set_continuity_counter(5);
        assert_eq!(packet.continuity_counter(), 5);
        packet.zero_continuity_counter();
        assert_eq!(packet.continuity_counter(), 0);
    }

    #[test]
    fn test_adaptation_field_0() {
        let mut p = PacketInfo::null_packet();
        let length: u8 = 0;
        p.set_adaptation_field(length);
        assert!(p.has_adaptation_field());
        assert_eq!(p.adaptation_field(), length);
    }

    #[test]
    fn test_adaptation_field_1() {
        let mut p = PacketInfo::null_packet();
        let length: u8 = 1;
        p.set_adaptation_field(length);
        assert!(p.has_adaptation_field());
        assert_eq!(p.adaptation_field(), length);
    }

    #[test]
    fn test_adaptation_field_2() {
        let mut p = PacketInfo::null_packet();
        let length: u8 = 2;
        p.set_adaptation_field(length);
        assert!(p.has_adaptation_field());
        assert_eq!(p.adaptation_field(), length);
    }

    #[test]
    fn test_scrambling_clear() {
        let mut p = PacketInfo::null_packet();
        p.set_scrambling(packet::SCRAMBLING_CLEAR);
        assert_eq!(p.scrambling(), packet::SCRAMBLING_CLEAR);
    }

    #[test]
    fn test_scrambling_even() {
        let mut p = PacketInfo::null_packet();
        p.set_scrambling(packet::SCRAMBLING_EVEN);
        assert_eq!(p.scrambling(), packet::SCRAMBLING_EVEN);
    }

    #[test]
    fn test_scrambling_odd() {
        let mut p = PacketInfo::null_packet();
        p.set_scrambling(packet::SCRAMBLING_ODD);
        assert_eq!(p.scrambling(), packet::SCRAMBLING_ODD);
    }

    #[test]
    fn test_discontinuity() {
        let mut p = PacketInfo::null_packet();
        p.set_discontinuity();
        assert!(p.has_discontinuity());
        p.clear_discontinuity();
        assert!(!p.has_discontinuity());
    }

    #[test]
    fn test_random_access() {
        let mut p = PacketInfo::null_packet();
        p.set_random_access();
        assert!(p.has_random_access());
    }

    #[test]
    fn test_stream_priority() {
        let mut p = PacketInfo::null_packet();
        p.set_stream_priority();
        assert!(p.has_stream_priority());
    }

    #[test]
    fn test_pcr() {
        let mut p = PacketInfo::null_packet();
        let pcr: u64 = 23647;
        p.set_pcr(pcr);
        assert!(p.has_pcr());
        assert_eq!(p.pcr(), pcr);
    }

    #[test]
    fn test_pcr_ext() {
        let mut p = PacketInfo::null_packet();
        let ext: u16 = 137;
        p.set_pcr_ext(ext);
        assert_eq!(p.pcr_ext(), ext);
    }
}
