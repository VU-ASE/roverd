// use tracing::info;

use axum::async_trait;

use openapi::{apis::services::*, models};

use openapi::models::*;

use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::{CookieJar, Multipart};

use tracing::warn;

use crate::state::Roverd;
use crate::warn_generic;

#[async_trait]
impl Services for Roverd {
    /// Fetches the zip file from the given URL and installs the service onto the filesystem.
    ///
    /// FetchPost - POST /fetch
    async fn fetch_post(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _body: models::FetchPostRequest,
    ) -> Result<FetchPostResponse, String> {
        Err("unimplemented".to_string())
    }

    /// Upload a new service or new version to the rover by uploading a ZIP file.
    ///
    /// UploadPost - POST /upload
    async fn upload_post(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _body: Multipart,
    ) -> Result<UploadPostResponse, String> {
        Err("unimplemented".to_string())
    }

    /// Retrieve the list of parsable service versions for a specific author and service.
    ///
    /// ServicesAuthorServiceGet - GET /services/{author}/{service}
    async fn services_author_service_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: ServicesAuthorServiceGetPathParams,
    ) -> Result<ServicesAuthorServiceGetResponse, String> {
        let state = self.state.read().await;
        let versions = warn_generic!(
            state.services.get_versions(path_params).await,
            ServicesAuthorServiceGetResponse
        );

        Ok(ServicesAuthorServiceGetResponse::Status200_TheListOfVersionsForThisAuthorAndServiceName(versions))
    }

    /// Delete a specific version of a service.
    ///
    /// ServicesAuthorServiceVersionDelete - DELETE /services/{author}/{service}/{version}
    async fn services_author_service_version_delete(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: ServicesAuthorServiceVersionDeletePathParams,
    ) -> Result<ServicesAuthorServiceVersionDeleteResponse, String> {
        let state = self.state.write().await;

        let rebuild_pipeline = warn_generic!(
            state.services.delete(&state, &path_params).await,
            ServicesAuthorServiceVersionDeleteResponse
        );

        Ok(ServicesAuthorServiceVersionDeleteResponse::Status200_TheServiceVersionWasDeletedSuccessfully(
            ServicesAuthorServiceVersionDelete200Response {
                invalidated_pipeline: rebuild_pipeline
            }
        ))
    }

    /// Retrieve the status of a specific version of a service.
    ///
    /// ServicesAuthorServiceVersionGet - GET /services/{author}/{service}/{version}
    async fn services_author_service_version_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: ServicesAuthorServiceVersionGetPathParams,
    ) -> Result<ServicesAuthorServiceVersionGetResponse, String> {
        let state = self.state.read().await;

        let service = warn_generic!(
            state.services.get_service(path_params).await,
            ServicesAuthorServiceVersionGetResponse
        );

        Ok(
            ServicesAuthorServiceVersionGetResponse::Status200_TheServiceConfiguration(
                models::ServicesAuthorServiceVersionGet200Response {
                    inputs: service
                        .0
                        .inputs
                        .iter()
                        .map(|i| ServicesAuthorServiceVersionGet200ResponseInputsInner {
                            service: i.service.clone(),
                            streams: i.streams.clone(),
                        })
                        .collect::<Vec<_>>(),
                    built_at: Some(69), // Todo this needs to be kept track of by state
                    outputs: service.0.outputs,
                },
            ),
        )
    }

    /// Build a fully qualified service version.
    ///
    /// ServicesAuthorServiceVersionPost - POST /services/{author}/{service}/{version}
    async fn services_author_service_version_post(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: ServicesAuthorServiceVersionPostPathParams,
    ) -> Result<ServicesAuthorServiceVersionPostResponse, String> {
        let state = self.state.write().await;
        let _ = if let Err(e) = state.services.build_service(path_params).await {
            warn!("{:#?}", e);
            return Ok(
                ServicesAuthorServiceVersionPostResponse::Status400_TheBuildFailed(
                    ServicesAuthorServiceVersionPost400Response {
                        build_log: None,             // TODO
                        message: "todo".to_string(), // TODO
                    },
                ),
            );
        };

        Ok(ServicesAuthorServiceVersionPostResponse::Status200_TheServiceWasBuiltSuccessfully)
    }

    /// Retrieve the list of all authors that have parsable services. With these authors you can query further for services.
    ///
    /// ServicesGet - GET /services
    async fn services_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
    ) -> Result<ServicesGetResponse, String> {
        let state = self.state.read().await;

        let authors = warn_generic!(state.services.get_authors().await, ServicesGetResponse);

        Ok(ServicesGetResponse::Status200_TheListOfAuthors(authors))
    }

    /// Retrieve the list of parsable services for a specific author.
    ///
    /// ServicesAuthorGet - GET /services/{author}
    async fn services_author_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: ServicesAuthorGetPathParams,
    ) -> Result<ServicesAuthorGetResponse, String> {
        let state = self.state.read().await;

        let services = warn_generic!(
            state.services.get_services(path_params).await,
            ServicesAuthorGetResponse
        );

        Ok(ServicesAuthorGetResponse::Status200_TheListOfServicesForTheAuthor(services))
    }
}
