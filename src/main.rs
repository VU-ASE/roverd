use tracing::info;

use tower_http::cors::CorsLayer;

mod error;
use error::*;

mod log;

mod state;
use state::*;

const LISTEN_ADDRESS: &str = "0.0.0.0:80";

/// Todo:
///     - cargo doc
///     - Type State pattern
///     - Model all errors exlicitly

#[tokio::main]
async fn main() -> Result<(), Error> {
    log::init();
    info!("logging initialized");

    let app = Roverd::new();

    let router = openapi::server::new(app).layer(CorsLayer::permissive());
    let listener = tokio::net::TcpListener::bind(LISTEN_ADDRESS).await.unwrap();

    info!("listening on {}", LISTEN_ADDRESS);

    axum::serve(listener, router).await.unwrap();

    Ok(())
}
