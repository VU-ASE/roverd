use axum::extract::{Request, State};
use axum::http::{self, StatusCode};
use axum::response::Response;
use tracing::info;

use tower_http::cors::CorsLayer;

use axum::middleware::{self, Next};

// use axum::routing::Router;

mod error;
use error::*;

mod log;

mod state;
use state::*;

const LISTEN_ADDRESS: &str = "0.0.0.0:80";

async fn auth(
    State(state): State<Roverd>,
    req: Request,
    _next: Next,
) -> Result<Response, StatusCode> {
    info!("{:#?}", req);
    info!("{:#?}", state);

    // let auth_header = req.headers()
    //     .get(http::header::AUTHORIZATION)
    //     .and_then(|header| header.to_str().ok());

    Err(StatusCode::UNAUTHORIZED)

    // let auth_header = if let Some(auth_header) = auth_header {
    //     auth_header
    // } else {
    //     return Err(StatusCode::UNAUTHORIZED);
    // };

    // if let Some(current_user) = authorize_current_user(auth_header).await {
    //     // insert the current user into a request extension so the handler can
    //     // extract it
    //     req.extensions_mut().insert(current_user);
    //     Ok(next.run(req).await)
    // } else {
    //     Err(StatusCode::UNAUTHORIZED)
    // }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    log::init();
    info!("logging initialized");

    let app = Roverd::new();

    let router = openapi::server::new(app.clone())
    .layer(CorsLayer::permissive());
        // .layer(middleware::from_fn_with_state(app.clone(), auth))

    let listener = tokio::net::TcpListener::bind(LISTEN_ADDRESS).await.unwrap();

    info!("listening on {}", LISTEN_ADDRESS);

    axum::serve(listener, router).await.unwrap();

    Ok(())
}
