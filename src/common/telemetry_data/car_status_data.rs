use crate::PacketHeader;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CarStatusData {
    pub m_tractionControl: u8, // Traction control - 0 = off, 1 = medium, 2 = full
    pub m_antiLockBrakes: u8,  // 0 (off) - 1 (on)
    pub m_fuelMix: u8,         // Fuel mix - 0 = lean, 1 = standard, 2 = rich, 3 = max
    pub m_frontBrakeBias: u8,  // Front brake bias (percentage)
    pub m_pitLimiterStatus: u8, // Pit limiter status - 0 = off, 1 = on
    pub m_fuelInTank: f32,     // Current fuel mass
    pub m_fuelCapacity: f32,   // Fuel capacity
    pub m_fuelRemainingLaps: f32, // Fuel remaining in terms of laps (value on MFD)
    pub m_maxRPM: u16,         // Cars max RPM, point of rev limiter
    pub m_idleRPM: u16,        // Cars idle RPM
    pub m_maxGears: u8,        // Maximum number of gears
    pub m_drsAllowed: u8,      // 0 = not allowed, 1 = allowed
    pub m_drsActivationDistance: u16, // 0 = DRS not available, non-zero - DRS will be available
    // in [X] metres
    pub m_actualTyreCompound: u8, // F1 Modern - 16 = C5, 17 = C4, 18 = C3, 19 = C2, 20 = C1
    // 7 = inter, 8 = wet
    // F1 Classic - 9 = dry, 10 = wet
    // F2 – 11 = super soft, 12 = soft, 13 = medium, 14 = hard
    // 15 = wet
    pub m_visualTyreCompound: u8, // F1 visual (can be different from actual compound)
    // 16 = soft, 17 = medium, 18 = hard, 7 = inter, 8 = wet
    // F1 Classic – same as above
    // F2 ‘19, 15 = wet, 19 – super soft, 20 = soft
    // 21 = medium , 22 = hard
    pub m_tyresAgeLaps: u8,    // Age in laps of the current set of tyres
    pub m_vehicleFiaFlags: i8, // -1 = invalid/unknown, 0 = none, 1 = green
    // 2 = blue, 3 = yellow, 4 = red
    pub m_ersStoreEnergy: f32, // ERS energy store in Joules
    pub m_ersDeployMode: u8,   // ERS deployment mode, 0 = none, 1 = medium
    // 2 = hotlap, 3 = overtake
    pub m_ersHarvestedThisLapMGUK: f32, // ERS energy harvested this lap by MGU-K
    pub m_ersHarvestedThisLapMGUH: f32, // ERS energy harvested this lap by MGU-H
    pub m_ersDeployedThisLap: f32,      // ERS energy deployed this lap
    pub m_networkPaused: u8,            // Whether the car is paused in a network game
}

#[derive(Deserialize, Debug)]
pub struct PacketCarStatusData {
    pub m_header: PacketHeader,
    pub car_status_data: [CarStatusData; 22],
}
