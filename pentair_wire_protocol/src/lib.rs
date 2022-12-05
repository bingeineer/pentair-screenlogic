use std::io;
use log::info;
use serde::{Deserialize, Serialize};
use tokio::net::UdpSocket;

const PENTAIR_BROADCAST_HOST: &str = "255.255.255.255:1444";

#[derive(Default)]
pub struct PentairClient {
}

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


#[derive(Serialize, Deserialize, Debug)]
struct BroadcastResponse {
    chk: [u8; 4],
    ip1: u8,
    ip2: u8,
    ip3: u8,
    ip4: u8,
    port: u16,
    gt: u8,
    gs: u8,
}

impl BroadcastResponse {
    fn parse(data: [u8; 12]) -> io::Result<Self> {
        let broadcast_data: BroadcastResponse = bincode::deserialize(&data).unwrap();

        if broadcast_data.chk[0] != 2 {
            return Err(std::io::Error::new(io::ErrorKind::Other, "got unexpected error from pentair response - chk != 2"));
        }

        Ok(broadcast_data)
    }

    fn host(&self) -> String {
        format!("{}.{}.{}.{}:{}", self.ip1, self.ip2, self.ip3, self.ip4, self.port)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct BroadcastRequest {
    data: [u8; 8],
}

impl BroadcastRequest {
    fn new() -> Self {
        let mut arr = [0; 8];
        arr[0] = 1;

        Self { data: arr }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
