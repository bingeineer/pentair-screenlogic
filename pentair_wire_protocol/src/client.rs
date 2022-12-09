use log::info;
use std::io;
use tokio::net::UdpSocket;

use crate::messages::{BroadcastRequest, BroadcastResponse};

const PENTAIR_BROADCAST_HOST: &str = "255.255.255.255:1444";

#[derive(Default)]
pub struct PentairClient {}

impl PentairClient {
    pub async fn start(&self) -> io::Result<()> {
        let pentair_server_host = pentair_handshake().await?;
        Ok(())
    }
}

async fn pentair_handshake() -> io::Result<String> {
    let sock = UdpSocket::bind("0.0.0.0:8117").await?;
    sock.set_broadcast(true)?;
    sock.set_ttl(30)?;

    let broadcast_search = BroadcastRequest::new();
    let serialized_data = bincode::serialize(&broadcast_search).unwrap();

    info!("Sending broadcast query to {PENTAIR_BROADCAST_HOST}");
    _ = sock
        .send_to(&serialized_data, PENTAIR_BROADCAST_HOST)
        .await?;

    let mut rx_buf = [0; 12];
    _ = sock.recv_from(&mut rx_buf).await?;

    let broadcast_resp = BroadcastResponse::parse(rx_buf)?;
    info!("received {:?} from pentair", broadcast_resp);

    Ok(broadcast_resp.host())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
