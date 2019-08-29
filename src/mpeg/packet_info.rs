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

    pub fn set_payload(&mut self) {
        packet::set_payload(&mut self.packet);
    }

    pub fn has_payload(&self) -> bool {
        packet::has_payload(&self.packet)
    }

    /// Sets the PID. Max: 8191 (0x1fff)
    pub fn set_pid(&mut self, pid: u16) {
        packet::set_pid(&mut self.packet, pid);
    }

    pub fn pid(&self) -> u16 {
        packet::pid(&self.packet)
    }

    /// Set the continuity counter. Max: 15
    pub fn set_continuity_counter(&mut self, cc: u8) {
        packet::set_continuity_counter(&mut self.packet, cc);
    }

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
