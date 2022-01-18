use async_std::net::UdpSocket;
use bincode::deserialize;
use crate::telemetry_data::{packet_header::*, motion_data::PacketMotionData};

pub async fn event_loop() {
	loop {
        let mut buf = vec![0; 2048];
        let socket = UdpSocket::bind("127.0.0.1:20777").await.unwrap();
        let result = socket.recv_from(&mut buf).await;
        if result.is_err() {
            continue;
        }
        let (n, _peer) = result.unwrap();

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
        if n > 1000 {
            println!("{}", n);
        }
        if n == 1464 {
            println!("{:#?}", n);
            let decoded: Result<PacketMotionData, _> = deserialize(&buf);
            if decoded.is_err() {
                println!("{:#?}", decoded);
                continue;
            }

            let decoded: PacketMotionData = decoded.unwrap();
            println!("{:#?}", decoded);
        }
    }
}