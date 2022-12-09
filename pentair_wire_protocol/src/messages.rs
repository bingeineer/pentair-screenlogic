use serde::{Deserialize, Serialize};
use std::io;

#[derive(Serialize, Deserialize, Debug)]
pub struct BroadcastResponse {
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
    pub fn parse(data: [u8; 12]) -> io::Result<Self> {
        let broadcast_data: BroadcastResponse = bincode::deserialize(&data).unwrap();

        if broadcast_data.chk[0] != 2 {
            return Err(std::io::Error::new(
                io::ErrorKind::Other,
                "got unexpected error from pentair response - chk != 2",
            ));
        }

        Ok(broadcast_data)
    }

    pub fn host(&self) -> String {
        format!(
            "{}.{}.{}.{}:{}",
            self.ip1, self.ip2, self.ip3, self.ip4, self.port
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BroadcastRequest {
    data: [u8; 8],
}

impl BroadcastRequest {
    pub fn new() -> Self {
        let mut arr = [0; 8];
        arr[0] = 1;

        Self { data: arr }
    }
}
