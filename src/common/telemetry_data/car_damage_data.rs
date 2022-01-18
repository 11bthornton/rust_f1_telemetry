use crate::PacketHeader;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CarDamageData
{
	pub m_tyresWear : [f32; 4], 			// Tyre wear (percentage)
	pub m_tyresDamage : [u8; 4], 		// Tyre damage (percentage)
	pub m_brakesDamage : [u8; 4], 		// Brakes damage (percentage)
	pub m_frontLeftWingDamage :  u8,	// Front left wing damage (percentage)
	pub m_frontRightWingDamage : u8, 	// Front right wing damage (percentage)
	pub m_rearWingDamage : u8, 		// Rear wing damage (percentage)
	pub m_floorDamage :  u8,			// Floor damage (percentage)
	pub m_diffuserDamage :  u8,		// Diffuser damage (percentage)
	pub m_sidepodDamage :  	u8,		// Sidepod damage (percentage)
	pub m_drsFault :  	u8,			// Indicator for DRS fault, 0 = OK, 1 = fault
	pub m_gearBoxDamage :  	u8,		// Gear box damage (percentage)
	pub m_engineDamage :  	u8,		// Engine damage (percentage)
	pub m_engineMGUHWear :  u8,		// Engine wear MGU-H (percentage)
	pub m_engineESWear :  	u8,		// Engine wear ES (percentage)
	pub m_engineCEWear :  	u8,		// Engine wear CE (percentage)
	pub m_engineICEWear :  		u8,	// Engine wear ICE (percentage)
	pub m_engineMGUKWear :  u8,		// Engine wear MGU-K (percentage)
	pub m_engineTCWear :  	u8,		// Engine wear TC (percentage)
}

#[derive(Deserialize, Debug)]
pub struct PacketCarDamageData
{
	pub m_header : PacketHeader, // Header
	pub m_carDamageData : [CarDamageData; 22],
}