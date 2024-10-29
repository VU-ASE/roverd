use tracing::info;

mod log;

mod state;

const LISTEN_ADDRESS: &str = "0.0.0.0:80";

#[tokio::main]
async fn main() -> Result<(), ()> {
    log::init();
    info!("logging initialized");

    let app = state::Roverd::new(String::from("my app"));
    info!("created state: {:?}", app);

    let router = openapi::server::new(app);
    let listener = tokio::net::TcpListener::bind(LISTEN_ADDRESS).await.unwrap();

    info!("listening on {}", LISTEN_ADDRESS);

    axum::serve(listener, router).await.unwrap();

    Ok(())
}
