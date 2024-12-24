use axum::extract::{Request, State};
use axum::http::{self, StatusCode};
use axum::middleware::{self, Next};
use axum::response::Response;
use base64::Engine;
use sha256::digest;
use tower_http::cors::CorsLayer;
use tracing::{info, warn};

mod apis;
mod command;
mod constants;
mod error;
mod log;
mod state;
mod util;

#[cfg(test)]
mod test;

use constants::*;
use error::Error::*;
use error::*;
use state::*;

/// TODO: this is not ideal, since middleware::from_fn_with_state expects
/// Result<Response, StatusCode>. But ideally, we want to use custom Error to
/// utilize ? more.
async fn auth_wrapper(
    state: State<Roverd>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    match auth(state, req, next).await {
        Ok(response) => Ok(response),
        Err(e) => {
            warn!("Unauthorized or bad request");
            match e {
                Http(status_code) => Err(status_code),
                _ => Err(StatusCode::BAD_REQUEST),
            }
        }
    }
}

fn check_auth(state: &Roverd, auth_str: &str) -> Result<(), Error> {
    let (user, password) = auth_str
        .split_once(':')
        .ok_or(Http(StatusCode::BAD_REQUEST))?;

    let stored_hash = digest(password);

    if let Some(hash) = &state.info.password {
        if user == state.info.username && hash == &stored_hash {
            return Ok(());
        }
    }

    warn!(
        "{}",
        format!(
            "Unauthorized access denied: missing credentials from {}",
            ROVER_INFO_FILE
        )
        .as_str()
    );

    Err(Http(StatusCode::UNAUTHORIZED))
}

async fn auth(State(state): State<Roverd>, req: Request, next: Next) -> Result<Response, Error> {
    info!("incoming {} on {}", req.method(), *req.uri());

    // the /status endpoint does not require authentication, all others do.
    if *req.uri() != *"/status" {
        let auth_header = req
            .headers()
            .get(http::header::AUTHORIZATION)
            .and_then(|header| header.to_str().ok())
            .ok_or(Http(StatusCode::UNAUTHORIZED))?;

        let basic_auth: Vec<&str> = auth_header.split(' ').collect();

        if basic_auth.len() != 2 || basic_auth[0] != "Basic" {
            warn!("request is missing basic auth header");
            return Err(Http(StatusCode::BAD_REQUEST));
        }

        let base64_data = basic_auth[1];

        let raw_bytes = base64::prelude::BASE64_STANDARD
            .decode(base64_data)
            .map_err(|_| Http(StatusCode::BAD_REQUEST))?;

        let auth_str =
            core::str::from_utf8(&raw_bytes).map_err(|_| Http(StatusCode::BAD_REQUEST))?;

        check_auth(&state, auth_str)?;
    }

    let response = next.run(req).await;
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    log::init();
    info!("logging initialized");

    let rover_state = Roverd::new().await?;

    let router = openapi::server::new(rover_state.clone())
        .layer(middleware::from_fn_with_state(
            rover_state.clone(),
            auth_wrapper,
        ))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind(LISTEN_ADDRESS).await.unwrap();

    info!("listening on {}", LISTEN_ADDRESS);

    axum::serve(listener, router).await.unwrap();

    Ok(())
}
