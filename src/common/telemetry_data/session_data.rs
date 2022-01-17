use crate::PacketHeader;
use serde::{Deserialize, Serialize};

use std::fmt; use std::marker::PhantomData;
use serde::ser::{Serializer, SerializeTuple};
use serde::de::{Deserializer, Visitor, SeqAccess, Error};
use derivative;

#[macro_use]
use serde_big_array::BigArray;

#[derive(Deserialize, Debug)]
pub struct MarshalZone
{
	pub m_zoneStart: f32,
	pub m_zoneFlag: i8,
}

#[derive(Deserialize, Debug, Serialize, Default, Copy, Clone)]
pub struct WeatherForecastSample
{
	pub session_type: u8,
	pub time_offset: u8,
	pub weather: u8,

	pub track_temperature: i8,
	pub track_temperature_change: i8,
	pub air_temperature: i8,
	pub air_temperature_change: i8,

	pub rain_percentage: u8,
}


#[derive(Deserialize, Debug)]
pub struct PacketSessionData
{
    pub m_header : PacketHeader,               	// Header

    pub m_weather : u8,              	// Weather - 0 = clear, 1 = light cloud, 2 = overcast
                                            	// 3 = light rain, 4 = heavy rain, 5 = storm
 	pub m_trackTemperature: i8,    	// Track temp. in degrees celsius
 	pub m_airTemperature: i8,      	// Air temp. in degrees celsius
    pub m_totalLaps: u8,           	// Total number of laps in this race
    pub m_trackLength: u16,           	// Track length in metres
    pub m_sessionType: u8,         	// 0 = unknown, 1 = P1, 2 = P2, 3 = P3, 4 = Short P
                                            	// 5 = Q1, 6 = Q2, 7 = Q3, 8 = Short Q, 9 = OSQ
                                            	// 10 = R, 11 = R2, 12 = R3, 13 = Time Trial
    pub m_trackId : i8,         		// -1 for unknown, 0-21 for tracks, see appendix
    pub m_formula : u8,                 	// Formula, 0 = F1 Modern, 1 = F1 Classic, 2 = F2,
                                                 // 3 = F1 Generic
    pub m_sessionTimeLeft : u16,    	// Time left in session in seconds
    pub m_sessionDuration : u16,    	// Session duration in seconds
    pub m_pitSpeedLimit : u8,      	// Pit speed limit in kilometres per hour
    pub m_gamePaused : u8,              // Whether the game is paused
    pub m_isSpectating : u8,        	// Whether the player is spectating
    pub m_spectatorCarIndex : u8, 	// Index of the car being spectated
    pub m_sliProNativeSupport : u8,	// SLI Pro support, 0 = inactive, 1 = active
    pub m_numMarshalZones : u8,        	// Number of marshal zones to follow
    pub m_marshalZones : [MarshalZone; 21],         	// List of marshal zones – max 21
    pub m_safetyCarStatus : u8,          // 0 = no safety car, 1 = full
                                                 // 2 = virtual, 3 = formation lap
    pub m_networkGame : u8,             // 0 = offline, 1 = online
    pub m_numWeatherForecastSamples : u8, // Number of weather samples to follow

    #[serde(with = "BigArray")]
    pub m_weatherForecastSamples : [WeatherForecastSample; 56],   // Array of weather forecast samples
    pub m_forecastAccuracy : u8,          // 0 = Perfect, 1 = Approximate
    pub m_aiDifficulty : u8,              // AI Difficulty rating – 0-110
    pub m_seasonLinkIdentifier : u32,      // Identifier for season - persists across saves
    pub m_weekendLinkIdentifier : u32,    // Identifier for weekend - persists across saves
    pub m_sessionLinkIdentifier : u32,    // Identifier for session - persists across saves
    pub m_pitStopWindowIdealLap : u8,    // Ideal lap to pit on for current strategy (player)
    pub m_pitStopWindowLatestLap : u8,   // Latest lap to pit on for current strategy (player)
    pub m_pitStopRejoinPosition : u8,   // Predicted position to rejoin at (player)
    pub m_steeringAssist : u8,            // 0 = off, 1 = on
    pub m_brakingAssist : u8,            // 0 = off, 1 = low, 2 = medium, 3 = high
    pub m_gearboxAssist : u8,             // 1 = manual, 2 = manual & suggested gear, 3 = auto
    pub m_pitAssist : u8,                // 0 = off, 1 = on
    pub m_pitReleaseAssist : u8,          // 0 = off, 1 = on
    pub m_ERSAssist : u8,                // 0 = off, 1 = on
    pub m_DRSAssist : u8,               // 0 = off, 1 = on
    pub m_dynamicRacingLine : u8,         // 0 = off, 1 = corners only, 2 = full
    pub m_dynamicRacingLineType : u8    // 0 = 2D, 1 = 3D
}
