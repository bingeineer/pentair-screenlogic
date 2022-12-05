use log::info;
use pentair_wire_protocol::PentairClient;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::init();

    info!("Connecting to pentair device...");
    let client = PentairClient::default();
    client.start().await?;

    Ok(())
}
