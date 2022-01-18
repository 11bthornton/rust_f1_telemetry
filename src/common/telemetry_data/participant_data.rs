use crate::PacketHeader;
use serde::Deserialize;
use std::str;

#[macro_use]
use serde_big_array::BigArray;



#[derive(Deserialize, Debug)]
pub struct ParticipantData {
	pub ai_controlled: u8,
	pub driver_id: u8,
	pub network_id: u8,
	pub team_id: u8,
	pub my_team: u8,
	pub race_number: u8,
	pub nationality: u8,

    #[serde(with = "BigArray")]
    pub name: [u8; 48],

    pub telemetry: u8,
}



#[derive(Deserialize, Debug)]
pub struct PacketParticipantData {

	pub m_header : PacketHeader,
	pub m_numActiveCars : u8,
	pub m_participants : [ParticipantData; 22]

}

impl ParticipantData {
	pub fn name(&self) -> &str {

		let name = str::from_utf8(&self.name).unwrap();

		name

	}
}