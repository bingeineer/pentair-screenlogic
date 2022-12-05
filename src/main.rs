use log::info;
use serde::{Deserialize, Serialize};
use std::io;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::init();

    info!("listening on localhost:8080");

    let sock = UdpSocket::bind("0.0.0.0:8080").await?;
    sock.set_broadcast(true)?;
    sock.set_ttl(30)?;

    let broadcast_search = BroadcastMessage::new();
    let serialized_data = bincode::serialize(&broadcast_search).unwrap();

    let len = sock
        .send_to(&serialized_data, "255.255.255.255:1444")
        .await?;
    info!("{:?} bytes sent", len);

    let mut rx_buf = [0; 12];

    let (len, addr) = sock.recv_from(&mut rx_buf).await?;
    info!("Received {:?} bytes from {:?}", len, addr);

    let info: BroadcastReceivedMessage = bincode::deserialize(&rx_buf).unwrap();
    info!("Received {:?}", info);

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct BroadcastReceivedMessage {
    chk: [u8; 4],
    ip1: u8,
    ip2: u8,
    ip3: u8,
    ip4: u8,
    port: u16,
    gt: u8,
    gs: u8,
}

#[derive(Serialize, Deserialize, Debug)]
struct BroadcastMessage {
    data: [u8; 8],
}

impl BroadcastMessage {
    fn new() -> Self {
        let mut arr = [0; 8];
        arr[0] = 1;

        Self { data: arr }
    }
}
