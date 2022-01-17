use crate::PacketHeader;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LapData {
    pub m_lastLapTimeInMS: u32,    // Last lap time in milliseconds
    pub m_currentLapTimeInMS: u32, // Current time around the lap in milliseconds
    pub m_sector1TimeInMS: u16,    // Sector 1 time in milliseconds
    pub m_sector2TimeInMS: u16,    // Sector 2 time in milliseconds
    pub m_lapDistance: f32,        // Distance vehicle is around current lap in metres – could
    // be negative if line hasn’t been crossed yet
    pub m_totalDistance: f32, // Total distance travelled in session in metres – could
    // be negative if line hasn’t been crossed yet
    pub m_safetyCarDelta: f32,   // Delta in seconds for safety car
    pub m_carPosition: u8,       // Car race position
    pub m_currentLapNum: u8,     // Current lap number
    pub m_pitStatus: u8,         // 0 = none, 1 = pitting, 2 = in pit area
    pub m_numPitStops: u8,       // Number of pit stops taken in this race
    pub m_sector: u8,            // 0 = sector1, 1 = sector2, 2 = sector3
    pub m_currentLapInvalid: u8, // Current lap invalid - 0 = valid, 1 = invalid
    pub m_penalties: u8,         // Accumulated time penalties in seconds to be added
    pub m_warnings: u8,          // Accumulated number of warnings issued
    pub m_numUnservedDriveThroughPens: u8, // Num drive through pens left to serve
    pub m_numUnservedStopGoPens: u8, // Num stop go pens left to serve
    pub m_gridPosition: u8,      // Grid position the vehicle started the race in
    pub m_driverStatus: u8,      // Status of driver - 0 = in garage, 1 = flying lap
    // 2 = in lap, 3 = out lap, 4 = on track
    pub m_resultStatus: u8, // Result status - 0 = invalid, 1 = inactive, 2 = active
    // 3 = finished, 4 = didnotfinish, 5 = disqualified
    // 6 = not classified, 7 = retired
    pub m_pitLaneTimerActive: u8, // Pit lane timing, 0 = inactive, 1 = active
    pub m_pitLaneTimeInLaneInMS: u16, // If active, the current time spent in the pit lane in ms
    pub m_pitStopTimerInMS: u16,  // Time of the actual pit stop in ms
    pub m_pitStopShouldServePen: u8, // Whether the car should serve a penalty at this stop
}

#[derive(Deserialize, Debug)]
pub struct PacketLapData {
    pub m_header: PacketHeader,   // Header
    pub m_lapData: [LapData; 22], // Lap data for all cars on track
}
