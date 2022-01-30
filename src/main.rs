use async_std::net::UdpSocket;
use bincode::{deserialize, serialize};
use serde::Deserialize;
use serde_bytes_repr::{ByteFmtDeserializer, ByteFmtSerializer};

mod common;

use common::telemetry_data::{
    car_damage_data::PacketCarDamageData,
    car_setup_data::PacketCarSetupData,
    car_status_data::PacketCarStatusData,
    car_telemetry_data::PacketCarTelemetryData,
    event_data::{PacketEventData, PacketEventFinal},
    lap_data::PacketLapData,
    motion_data::PacketMotionData,
    packet_header::*,
    participant_data::PacketParticipantData,
    session_data::PacketSessionData,
};

#[tokio::main]
async fn main() {
    let mut vec: Vec<usize> = vec![];
    loop {
        let mut buf = vec![0; 2048];
        let socket = UdpSocket::bind("127.0.0.1:20777").await.unwrap();
        let result = socket.recv_from(&mut buf).await;
        if result.is_err() {
            panic!("Panicking here.... ");
        }
        let (n, peer) = result.unwrap();

        if (!vec.contains(&n)) {
            println!("{}", n);
            vec.push(n);
        }

        // if n == 970 {
        //     println!("{:#?}", n);
        //     let decoded: Result<PacketLapData, _> = deserialize(&buf);
        //     if decoded.is_err() {
        //         continue;
        //     }

        //     let decoded: PacketLapData = decoded.unwrap();
        //     let index = decoded.m_header.m_playerCarIndex;
        //     println!("{:#?}", decoded.m_lapData[index as usize]);
        // }
        // if n == 625 {
        //     println!("{:#?}", n);
        //     let decoded: Result<PacketSessionData, _> = deserialize(&buf);
        //     if decoded.is_err() {
        //         println!("{:#?}", decoded);
        //         continue;
        //     }

        //     let decoded: PacketSessionData = decoded.unwrap();
        //     println!("{:#?}", decoded);
        // }

        if n == 1058 {
            let decoded: Result<PacketCarStatusData, _> = deserialize(&buf);
            if decoded.is_err() {
                panic!();
            }

            let decoded: PacketCarStatusData = decoded.unwrap();
            let index = decoded.m_header.m_playerCarIndex;
            // let decoded = decoded.decode().unwrap();
            println!("{:#?}", decoded.car_status_data[index as usize]);
        }
    }
}
