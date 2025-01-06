use axum::extract::{DefaultBodyLimit, Request, State};
use axum::http::{self, StatusCode};
use axum::middleware::{self, Next};
use axum::response::Response;
use base64::Engine;
use openapi::models::DaemonStatus;
use std::fs::{self, Permissions};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use tower_http::cors::CorsLayer;
use tracing::{error, info, warn};

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

/// Not ideal, but an error wrapper work around since middleware::from_fn_with_state expects
/// Result<Response, StatusCode>. But ideally, we want to use custom Error to propogate our custom
/// Error type as mmuch as possible.
async fn auth_wrapper(
    state: State<Roverd>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    match auth(state, req, next).await {
        Ok(response) => Ok(response),
        Err(e) => {
            warn!("Unauthorized or bad request: {:?}", e);
            match e {
                Http(status_code) => Err(status_code),
                _ => Err(StatusCode::BAD_REQUEST),
            }
        }
    }
}

/// Performs password check to hashed password stored on disk.
fn check_auth(state: &Roverd, auth_str: &str) -> Result<(), Error> {
    let (user, password) = auth_str
        .split_once(':')
        .ok_or(Http(StatusCode::BAD_REQUEST))?;

    let stored_hash = sha256::digest(password);

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

/// Main authentication logic requires authenticated requests for all endpoints
/// except for "/status".
async fn auth(State(state): State<Roverd>, req: Request, next: Next) -> Result<Response, Error> {
    info!("{} {}", req.method(), *req.uri());

    // the /status endpoint does not require authentication, all others do.
    if *req.uri() != *"/status" {
        if state.info.status == DaemonStatus::Operational {
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

            // Returns early if authentication fails
            check_auth(&state, auth_str)?;
        } else {
            return Err(Error::RoverdNotOperational);
        }
    }

    // Pass the request on to the request handlers.
    let response = next.run(req).await;
    Ok(response)
}

/// Entry of program, initializes logging and constructs app state used by axum router.
#[tokio::main]
async fn main() -> Result<(), Error> {
    log::init();
    info!("logging initialized");

    // Download latest version of roverd from github
    match download_latest_roverd().await {
        Err(e) => {
            error!("Unable to download latest roverd");
            error!("{:?}", e);
        }
        _ => (),
    };

    // All app initialization happens in new()
    let rover_state = Roverd::new().await?;

    // Hand-off to axum with a max upload limit of 100MB
    let router = openapi::server::new(rover_state.clone())
        .layer(middleware::from_fn_with_state(
            rover_state.clone(),
            auth_wrapper,
        ))
        .layer(CorsLayer::permissive())
        .layer(DefaultBodyLimit::max(100000000));

    let listener = tokio::net::TcpListener::bind(LISTEN_ADDRESS).await.unwrap();

    info!("listening on {}", LISTEN_ADDRESS);

    axum::serve(listener, router).await.unwrap();

    Ok(())
}

async fn download_latest_roverd() -> Result<(), Error> {
    info!("downloading latest roverd");

    let response = reqwest::get(ROVERD_DOWNLOAD_URL).await?;
    info!("let response = reqwest::get(ROVERD_DOWNLOAD_URL).await?;");

    let new_binary = response.bytes().await?;
    info!("let new_binary = response.bytes().await?;");

    let mut roverd_file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(ROVERD_INSTALL_PATH)?;

    info!("let new_binary = response.bytes().await?;");

    roverd_file.write_all(&new_binary)?;

    fs::set_permissions(ROVERD_INSTALL_PATH, Permissions::from_mode(0o755))?;
    info!("fs::set_permissions(temp_path.clone(), Permissions::from_mode(0o755))?;");

    Ok(())
}
