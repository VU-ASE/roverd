mod log;

mod error;
use error::Result;

use tracing::info;

const LISTEN_ADDRESS: &str = "0.0.0.0:80";

#[tokio::main]
async fn main() -> Result<()> {
    log::init();
    info!("{}", LISTEN_ADDRESS);
    Ok(())
}
