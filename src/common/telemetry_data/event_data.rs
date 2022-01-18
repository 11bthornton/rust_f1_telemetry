use std::str;

use crate::PacketHeader;
use serde::Deserialize;
use bincode::{deserialize, serialize};
use serde_bytes_repr::{ByteFmtDeserializer, ByteFmtSerializer};
use std::str::Utf8Error;

#[derive(Deserialize, Debug)]
pub struct PacketEventData {

	pub m_header : PacketHeader,
	pub m_eventStringCode : [u8; 4],
	pub remaining_data : [u8; 16]
}

impl PacketEventData {

	pub fn decode(self) -> Result<PacketEventFinal, Utf8Error> {

		let button_code = str::from_utf8(&self.m_eventStringCode)?;

		match button_code {
			"BUTN" => {

				let decoded : Buttons = deserialize(&self.remaining_data).unwrap();

				Ok(
					PacketEventFinal {
						m_header: self.m_header,
						m_eventStringCode: self.m_eventStringCode,
						r#type : EventType::Buttons(decoded)
					}
				)
			},
			"FLBK" => {

				let decoded : FlashBack = deserialize(&self.remaining_data).unwrap();

				Ok(
					PacketEventFinal {
						m_header: self.m_header,
						m_eventStringCode: self.m_eventStringCode,
						r#type : EventType::FlashBack(decoded)
					}
				)
			},
			"TMTP" => {

				let decoded : TeamMateInPits = deserialize(&self.remaining_data).unwrap();

				Ok(
					PacketEventFinal {
						m_header: self.m_header,
						m_eventStringCode: self.m_eventStringCode,
						r#type : EventType::TeamMateInPits(decoded)
					}
				)
			},
			"RCWN" => {

				let decoded : RaceWinner = deserialize(&self.remaining_data).unwrap();

				Ok(
					PacketEventFinal {
						m_header: self.m_header,
						m_eventStringCode: self.m_eventStringCode,
						r#type : EventType::RaceWinner(decoded)
					}
				)
			},
			"RTMT" => {

				let decoded : Retirement = deserialize(&self.remaining_data).unwrap();

				Ok(
					PacketEventFinal {
						m_header: self.m_header,
						m_eventStringCode: self.m_eventStringCode,
						r#type : EventType::Retirement(decoded)
					}
				)
			},
			"FTLP" => {

				let decoded : FastestLap = deserialize(&self.remaining_data).unwrap();

				Ok(
					PacketEventFinal {
						m_header: self.m_header,
						m_eventStringCode: self.m_eventStringCode,
						r#type : EventType::FastestLap(decoded)
					}
				)
			},
			"STLG" => {

				let decoded : StartLights = deserialize(&self.remaining_data).unwrap();

				Ok(
					PacketEventFinal {
						m_header: self.m_header,
						m_eventStringCode: self.m_eventStringCode,
						r#type : EventType::StartLights(decoded)
					}
				)
			},
			"SPTP" => {

				let decoded : SpeedTrap = deserialize(&self.remaining_data).unwrap();

				Ok(
					PacketEventFinal {
						m_header: self.m_header,
						m_eventStringCode: self.m_eventStringCode,
						r#type : EventType::SpeedTrap(decoded)
					}
				)
			},
			"PENA" => {

				let decoded : Penalty = deserialize(&self.remaining_data).unwrap();

				Ok(
					PacketEventFinal {
						m_header: self.m_header,
						m_eventStringCode: self.m_eventStringCode,
						r#type : EventType::Penalty(decoded)
					}
				)
			},
			"DTSV" => {

				let decoded : DriveThroughPenaltyServed = deserialize(&self.remaining_data).unwrap();

				Ok(
					PacketEventFinal {
						m_header: self.m_header,
						m_eventStringCode: self.m_eventStringCode,
						r#type : EventType::DriveThroughPenaltyServed(decoded)
					}
				)
			},
			"SGSV" => {

				let decoded : StopGoPenaltyServed = deserialize(&self.remaining_data).unwrap();

				Ok(
					PacketEventFinal {
						m_header: self.m_header,
						m_eventStringCode: self.m_eventStringCode,
						r#type : EventType::StopGoPenaltyServed(decoded)
					}
				)
			},
			"LGOT" => {

				Ok(
					PacketEventFinal {
						m_header: self.m_header,
						m_eventStringCode: self.m_eventStringCode,
						r#type : EventType::LightsOut
					}
				)
			},
			"SSTA" => {

				Ok(
					PacketEventFinal {
						m_header: self.m_header,
						m_eventStringCode: self.m_eventStringCode,
						r#type : EventType::SessionStart
					}
				)
			},
			"SEND" => {

				Ok(
					PacketEventFinal {
						m_header: self.m_header,
						m_eventStringCode: self.m_eventStringCode,
						r#type : EventType::SessionEnd
					}
				)
			},
			"CHQF" => {

				Ok(
					PacketEventFinal {
						m_header: self.m_header,
						m_eventStringCode: self.m_eventStringCode,
						r#type : EventType::ChequeredFlag
					}
				)
			},
			"DRSE" => {

				Ok(
					PacketEventFinal {
						m_header: self.m_header,
						m_eventStringCode: self.m_eventStringCode,
						r#type : EventType::DrsEnabled
					}
				)
			},
			"DRSD" => {

				Ok(
					PacketEventFinal {
						m_header: self.m_header,
						m_eventStringCode: self.m_eventStringCode,
						r#type : EventType::DrsDisabled
					}
				)
			},

			_ => {
				panic!("Something gone wrong! {}", button_code)
			},

		}
	}

}

#[derive(Deserialize, Debug)]
pub struct PacketEventFinal {

	pub m_header : PacketHeader,
	pub m_eventStringCode : [u8; 4],
	pub r#type: EventType 

}

#[derive(Deserialize, Debug)]
pub struct Buttons {
	m_buttonStatus: u32,
}
#[derive(Deserialize, Debug)]
pub struct FastestLap {
	vehicle_index : u8,
	lap_time: f32
}
#[derive(Deserialize, Debug)]
pub struct Retirement {
	vehicle_index: u8,
}
#[derive(Deserialize, Debug)]
pub struct TeamMateInPits {
	vehicle_index: u8
}
#[derive(Deserialize, Debug)]
pub struct RaceWinner {
	vehicle_index: u8
}
#[derive(Deserialize, Debug)]
pub struct Penalty {
	pub penalty_type : u8,
	pub infringement_type: u8,
	pub vehicle_index: u8,
	pub time: u8,
	pub lap_number: u8,
	pub places_gained: u8,
}
#[derive(Deserialize, Debug)]
pub struct SpeedTrap {
	pub vehicle_index: u8,
	pub speed : f32,
	pub overall_fastest_in_session: u8,
	pub driver_fastest_in_session: u8,
}
#[derive(Deserialize, Debug)]
pub struct StartLights {
	pub num_lights : u8
}
#[derive(Deserialize, Debug)]
pub struct DriveThroughPenaltyServed {
	pub vehicle_index: u8,
}
#[derive(Deserialize, Debug)]
pub struct StopGoPenaltyServed {
	vehicle_index: u8
}
#[derive(Deserialize, Debug)]
pub struct FlashBack {
	pub flashback_frame_identifier : u32,
	pub flashback_session_time : f32
}

#[derive(Deserialize, Debug)]
pub enum EventType {
	Buttons(Buttons),
	FastestLap(FastestLap),
	Retirement(Retirement),
	TeamMateInPits(TeamMateInPits),
	RaceWinner(RaceWinner),
	Penalty(Penalty),
	SpeedTrap(SpeedTrap),
	StartLights(StartLights),
	DriveThroughPenaltyServed(DriveThroughPenaltyServed),
	StopGoPenaltyServed(StopGoPenaltyServed),
	FlashBack(FlashBack),
	LightsOut,
	SessionStart,
	SessionEnd,
	DrsEnabled,
	DrsDisabled,
	ChequeredFlag,
	AlternateSpeedTrap
}