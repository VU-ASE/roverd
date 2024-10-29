// use tracing::info;

use axum::async_trait;

use openapi::apis::health::*;

use openapi::models;

use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::CookieJar;

use crate::state::Roverd;

#[async_trait]
impl Health for Roverd {
    /// Retrieve the health and versioning information.
    ///
    /// StatusGet - GET /status
    async fn status_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
    ) -> Result<StatusGetResponse, String> {
        Ok(
            StatusGetResponse::Status200_TheHealthAndVersioningInformation(
                models::StatusGet200Response::new(),
            ),
        )
    }

    /// Self-update the roverd daemon process.
    ///
    /// UpdatePost - POST /update
    async fn update_post(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
    ) -> Result<UpdatePostResponse, String> {
        Ok(UpdatePostResponse::Status200_TheRoverdDaemonProcessInitiatedASelf)
    }
}
