use axum::extract::{DefaultBodyLimit, Request, State};
use axum::http::{self, StatusCode};
use axum::middleware::{self, Next};
use axum::response::Response;
use base64::Engine;
use sha256::digest;
use tower_http::cors::CorsLayer;
use tracing::{info, warn};

mod apis;
mod constants;
mod error;
mod log;
mod state;
mod util;

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
            warn!(">>> Unauthorized or bad request");
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
    info!("{}: {}", req.method(), *req.uri());

    // the /status endpoint does not require authentication, all others do.
    if *req.uri() != *"/status" {
        let auth_header = req
            .headers()
            .get(http::header::AUTHORIZATION)
            .and_then(|header| header.to_str().ok())
            .ok_or(Http(StatusCode::UNAUTHORIZED))?;

        let basic_auth: Vec<&str> = auth_header.split(' ').collect();

        if basic_auth.len() != 2 || basic_auth[0] != "Basic" {
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

#[cfg(test)]
mod test {
    use std::path::Path;

    use once_cell::sync::Lazy;
    use openapi::apis::services::ServicesGetResponse;
    use tokio::sync::OnceCell;

    use openapi::models::*;
    use reqwest::multipart;
    use reqwest::{header, Client, Response};

    use super::*;

    static SERVER: Lazy<OnceCell<Result<(), Error>>> = Lazy::new(OnceCell::new);

    async fn start_server() -> Result<(), Error> {
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

        tokio::spawn(async move {
            axum::serve(listener, router).await.unwrap();
        });

        Ok(())
    }

    async fn ensure_server_is_started() -> &'static Result<(), Error> {
        // TODO this doesn't seem to be working as expected, which is why 
        // all tests are in one funciton.
        SERVER.get_or_init(|| async { start_server().await }).await
    }

    fn create_authenticated_client() -> Client {
        Client::builder()
            .default_headers({
                let mut headers = header::HeaderMap::new();
                let auth_value = base64::encode("debix:debix");
                headers.insert(
                    header::AUTHORIZATION,
                    header::HeaderValue::from_str(&format!("Basic {}", auth_value)).unwrap(),
                );
                headers
            })
            .build()
            .expect("Failed to create authenticated client")
    }


    async fn check_status(client: &Client) -> Result<(), Error> {
        let response = client.get("http://localhost/status").send().await?;

        assert!(response.status().is_success());

        let response_string = response.text().await?;
        let r: StatusGet200Response = serde_json::from_str(&response_string)?;

        assert_eq!(r.status, DaemonStatus::Operational);
        assert_eq!(r.error_message, None);
        assert_eq!(r.version, String::from("0.1.0"));
        assert_eq!(r.os, String::from("Debian 12.0.0 [64-bit]"));
        assert_eq!(r.rover_id, Some(13));
        assert_eq!(r.rover_name, Some(String::from("bunny")));

        Ok(())
    }


    async fn delete_service(client: &Client, s: &str) -> Response {
        client.delete(s).send().await.unwrap()
    }
    
    async fn upload_file(client: &Client, s: &str) -> Result<Response, Error> {
        let file_path = Path::new(s);
        let form = multipart::Form::new()
            .file("file", file_path).await?;

        let response = client
            .post("http://localhost:/upload")  // Replace with your actual endpoint
            .multipart(form)
            .send()
            .await?;
        Ok(response)
    }


    #[tokio::test]
    async fn all() -> Result<(), Error> {
        ensure_server_is_started().await;
        let client = create_authenticated_client();

        check_status(&client).await?;

        // First, delete any example files, regardless if they are there or not 
        let _ = delete_service(&client, "http://localhost/services/vu-ase/imaging/1.0.0").await;
        let _ = delete_service(&client, "http://localhost/services/vu-ase/controller/1.0.0").await;
        let _ = delete_service(&client, "http://localhost/services/vu-ase/actuator/1.0.0").await;


        // Upload three services, this must succeed.
        let r = upload_file(&client, "examples/actuator.zip").await?;
        assert!(r.status().is_success());
        let r = upload_file(&client, "examples/imaging.zip").await?;
        assert!(r.status().is_success());
        let r = upload_file(&client, "examples/controller.zip").await?;
        assert!(r.status().is_success());


        // List all authors, must succeed
        let r = client.get("http://localhost/services").send().await?;
        assert!(r.status().is_success());
        let s = r.text().await?;
        let data: Vec<String> = serde_json::from_str(&s)?;
        assert_eq!(data, vec!["vu-ase"]);


        // List all services, must succeed
        let r = client.get("http://localhost/services/vu-ase").send().await?;
        assert!(r.status().is_success());
        let s = r.text().await?;
        let data: Vec<String> = serde_json::from_str(&s)?;
        assert_eq!(data, vec!["actuator", "controller", "imaging"]);


        // List all versions, must succeed
        let r = client.get("http://localhost/services/vu-ase/imaging").send().await?;
        assert!(r.status().is_success());
        let s = r.text().await?;
        let data: Vec<String> = serde_json::from_str(&s)?;
        assert_eq!(data, vec!["1.0.0",]);

        let r = client.get("http://localhost/services/vu-ase/controller").send().await?;
        assert!(r.status().is_success());
        let s = r.text().await?;
        let data: Vec<String> = serde_json::from_str(&s)?;
        assert_eq!(data, vec!["1.0.0",]);

        let r = client.get("http://localhost/services/vu-ase/actuator").send().await?;
        assert!(r.status().is_success());
        let s = r.text().await?;
        let data: Vec<String> = serde_json::from_str(&s)?;
        assert_eq!(data, vec!["1.0.0",]);

        


        Ok(())
    }
}
