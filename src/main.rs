use async_std::net::UdpSocket;
use serde::{Deserialize};
use serde_bytes_repr::{ByteFmtDeserializer, ByteFmtSerializer};
use bincode::{serialize, deserialize};

#[tokio::main]
async fn main() {
    
    loop {
        let mut buf = vec![0; 970];
        let socket = UdpSocket::bind("127.0.0.1:20777").await.unwrap();
        let result = socket.recv_from(&mut buf).await;
        if result.is_err() {
            continue;
        }
        let (n, peer) = result.unwrap();
        
        if n == 970 {
            println!("{:#?}", n);
            let decoded: Result<PacketLapData, _> = deserialize(&buf);
            if decoded.is_err() {
                continue;
            }

            let decoded : PacketLapData = decoded.unwrap();
            let index = decoded.m_header.m_playerCarIndex;
            println!("{:#?}", decoded.m_lapData[index as usize]);
        }
    }
        

}

#[derive(Deserialize, Debug)]
struct PacketHeader
{
    m_packetFormat: u16,
    m_gameMajorVersion: u8,
    m_gameMinorVersion: u8,
    m_packetVersion: u8,
    m_packetId: u8,
    m_sessionUID: u64,
    m_sessionTime: f32,
    m_frameIdentifier: u32,
    m_playerCarIndex: u8,
    m_secondaryPlayerCarIndex: u8
}

#[derive(Deserialize, Debug)]
struct LapData
{
    m_lastLapTimeInMS : u32,          // Last lap time in milliseconds
    m_currentLapTimeInMS : u32,   // Current time around the lap in milliseconds
    m_sector1TimeInMS: u16,  // Sector 1 time in milliseconds
    m_sector2TimeInMS: u16,            // Sector 2 time in milliseconds
    m_lapDistance: f32,      // Distance vehicle is around current lap in metres – could
                     // be negative if line hasn’t been crossed yet
    m_totalDistance: f32,        // Total distance travelled in session in metres – could
                     // be negative if line hasn’t been crossed yet
    m_safetyCarDelta: f32,            // Delta in seconds for safety car
    m_carPosition: u8,              // Car race position
    m_currentLapNum: u8,        // Current lap number
    m_pitStatus: u8,                 // 0 = none, 1 = pitting, 2 = in pit area
    m_numPitStops: u8,               // Number of pit stops taken in this race
    m_sector: u8,                  // 0 = sector1, 1 = sector2, 2 = sector3
    m_currentLapInvalid: u8,         // Current lap invalid - 0 = valid, 1 = invalid
    m_penalties: u8,                // Accumulated time penalties in seconds to be added
    m_warnings: u8,                  // Accumulated number of warnings issued
    m_numUnservedDriveThroughPens: u8,  // Num drive through pens left to serve
    m_numUnservedStopGoPens: u8,         // Num stop go pens left to serve
    m_gridPosition: u8,              // Grid position the vehicle started the race in
    m_driverStatus: u8,              // Status of driver - 0 = in garage, 1 = flying lap
                                          // 2 = in lap, 3 = out lap, 4 = on track
    m_resultStatus: u8,               // Result status - 0 = invalid, 1 = inactive, 2 = active
                                          // 3 = finished, 4 = didnotfinish, 5 = disqualified
                                          // 6 = not classified, 7 = retired
    m_pitLaneTimerActive: u8,       // Pit lane timing, 0 = inactive, 1 = active
    m_pitLaneTimeInLaneInMS: u16,        // If active, the current time spent in the pit lane in ms
    m_pitStopTimerInMS : u16,             // Time of the actual pit stop in ms
    m_pitStopShouldServePen: u8      // Whether the car should serve a penalty at this stop
}

#[derive(Deserialize, Debug)]
struct PacketLapData
{
    m_header: PacketHeader,              // Header
    m_lapData: [LapData; 22]
      // Lap data for all cars on track
}
