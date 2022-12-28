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

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginMsg {
    code1: u16,
    code2: u16,
    schema: u32,
    connection_type: u32,
    version: String,
    data_array: [u8; 16],
    process_id: u32,
}

impl LoginMsg {
    pub fn new() -> Self {
        Self {
            code1: 0,
            code2: 27,
            schema: 348,
            connection_type: 0,
            version: String::from("Android"),
            data_array: [0; 16],
            process_id: 2,
        }
    }
}
