use log::{debug, info};
use std::io;
use tokio::{
    io::AsyncWriteExt,
    net::{TcpStream, UdpSocket},
};

use crate::messages::{BroadcastRequest, BroadcastResponse, LoginMsg};

const PENTAIR_BROADCAST_HOST: &str = "255.255.255.255:1444";
const UDP_RESPONSE_HOST: &str = "0.0.0.0:8117";

#[derive(Default)]
pub struct PentairClient {}

impl PentairClient {
    pub async fn start(&self) -> io::Result<()> {
        let pentair_server_host = pentair_handshake().await?;

        info!("connecting to {pentair_server_host}");
        let mut stream = TcpStream::connect(pentair_server_host).await?;
        let (_read, mut write) = stream.split();
        let host_str: Vec<u8> = b"CONNECTSERVERHOST".to_vec();
        let conn_num: Vec<u8> = vec![13, 10, 13, 10];
        let res: Vec<u8> = [host_str, conn_num].concat();

        debug!("Sending connection message");
        write.write_all(&res).await?;

        // Message Codes: 0,27
        // Parameters:
        // • (int) Schema [use 348]
        // • (int) Connection type [use 0]
        // • (String) Client Version [use ‘Android’]
        // • (byte[ ]) Data [use array filled with zeros of length 16]
        // • (int) Process ID [use 2]

        let login_msg = bincode::serialize(&LoginMsg::new()).unwrap();
        write.write_all(&login_msg).await?;

        loop {
            stream.readable().await?;

            let mut buf = [0; 4096];
            match stream.try_read(&mut buf) {
                Ok(0) => {
                    debug!("received 0 bytes");
                    break;
                }
                Ok(n) => println!("Read {n} bytes"),
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    debug!("would block");
                    continue;
                }
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }
}

async fn pentair_handshake() -> io::Result<String> {
    let sock = UdpSocket::bind(UDP_RESPONSE_HOST).await?;
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

    Ok(broadcast_resp.host())
}
