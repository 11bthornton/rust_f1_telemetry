use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PacketHeader {
    pub m_packetFormat: u16,
    pub m_gameMajorVersion: u8,
    pub m_gameMinorVersion: u8,
    pub m_packetVersion: u8,
    pub m_packetId: u8,
    pub m_sessionUID: u64,
    pub m_sessionTime: f32,
    pub m_frameIdentifier: u32,
    pub m_playerCarIndex: u8,
    pub m_secondaryPlayerCarIndex: u8,
}
