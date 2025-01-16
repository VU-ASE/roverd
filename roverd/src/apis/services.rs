// use tracing::info;

use axum::async_trait;

use openapi::{apis::services::*, models};

use openapi::models::*;

use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::{CookieJar, Multipart};

use serde_json::value::RawValue;
use tracing::warn;

use crate::app::Roverd;
use crate::error::Error;
use crate::service::FqBuf;
use crate::{rover_is_operating, warn_generic};

#[async_trait]
impl Services for Roverd {
    /// Fetches the zip file from the given URL and installs the service onto the filesystem.
    /// `RoverState` - This function can run *only when dormant*
    /// TODO: fs_lock
    /// FetchPost - POST /fetch
    async fn fetch_post(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        body: models::FetchPostRequest,
    ) -> Result<FetchPostResponse, ()> {
        if let Some(rover_state) = self.try_get_dormant().await {
            let (fq_buf, invalidated_pipeline) = warn_generic!(
                self.app.fetch_service(&body, rover_state).await,
                FetchPostResponse
            );

            Ok(
                FetchPostResponse::Status200_TheServiceWasUploadedSuccessfully(
                    FetchPost200Response {
                        name: fq_buf.name,
                        author: fq_buf.author,
                        version: fq_buf.version,
                        invalidated_pipeline,
                    },
                ),
            )
        } else {
            rover_is_operating!(FetchPostResponse)
        }
    }

    /// Upload a new service or new version to the rover by uploading a ZIP file.
    /// `RoverState` - This function can run *only when dormant*
    /// TODO: fs_lock
    /// UploadPost - POST /upload
    async fn upload_post(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        body: Multipart,
    ) -> Result<UploadPostResponse, ()> {
        if let Some(rover_state) = self.try_get_dormant().await {
            let (fq_buf, invalidated_pipeline) = warn_generic!(
                self.app.receive_upload(body, rover_state).await,
                UploadPostResponse
            );

            Ok(
                UploadPostResponse::Status200_TheServiceWasUploadedSuccessfully(
                    FetchPost200Response {
                        name: fq_buf.name,
                        author: fq_buf.author,
                        version: fq_buf.version,
                        invalidated_pipeline,
                    },
                ),
            )
        } else {
            rover_is_operating!(UploadPostResponse)
        }
    }

    /// Retrieve the list of parsable service versions for a specific author and service.
    /// `RoverState` - This function can run *always*
    /// TODO: fs_lock
    /// ServicesAuthorServiceGet - GET /services/{author}/{service}
    async fn services_author_service_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: ServicesAuthorServiceGetPathParams,
    ) -> Result<ServicesAuthorServiceGetResponse, ()> {
        let versions = warn_generic!(
            self.app.get_versions(path_params).await,
            ServicesAuthorServiceGetResponse
        );

        Ok(ServicesAuthorServiceGetResponse::Status200_TheListOfVersionsForThisAuthorAndServiceName(versions))
    }

    /// Delete a specific version of a service.
    /// `RoverState` - This function can run *only when dormant*
    /// TODO: fs_lock
    /// ServicesAuthorServiceVersionDelete - DELETE /services/{author}/{service}/{version}
    async fn services_author_service_version_delete(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: ServicesAuthorServiceVersionDeletePathParams,
    ) -> Result<ServicesAuthorServiceVersionDeleteResponse, ()> {
        if let Some(rover_state) = self.try_get_dormant().await {
            let invalidated_pipeline = warn_generic!(
                self.app.delete_service(&path_params, rover_state).await,
                ServicesAuthorServiceVersionDeleteResponse
            );

            Ok(ServicesAuthorServiceVersionDeleteResponse::Status200_TheServiceVersionWasDeletedSuccessfully(
                ServicesAuthorServiceVersionDelete200Response {
                    invalidated_pipeline
                }
            ))
        } else {
            rover_is_operating!(ServicesAuthorServiceVersionDeleteResponse)
        }
    }

    /// Retrieve the status of a specific version of a service.
    /// `RoverState` - This function can run *always*
    /// TODO: fs_lock
    /// ServicesAuthorServiceVersionGet - GET /services/{author}/{service}/{version}
    async fn services_author_service_version_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: ServicesAuthorServiceVersionGetPathParams,
    ) -> Result<ServicesAuthorServiceVersionGetResponse, ()> {
        let fq = FqBuf::from(&path_params);

        let service = warn_generic!(
            self.app.get_service(fq.clone()).await,
            ServicesAuthorServiceVersionGetResponse
        );

        let built_services = self.app.built_services.read().await;
        let built_at = built_services.get(&fq).copied();

        let mut configuration = vec![];

        for c in service.0.configuration.iter() {
            configuration.push(models::ServicesAuthorServiceVersionGet200ResponseConfigurationInner {
                name: c.name.clone(),
                r#type: match c.value.clone() {
                    rovervalidate::service::Value::Number(_) => "number".to_string(),
                    rovervalidate::service::Value::String(_) => "string".to_string(),
                },
                tunable: c.tunable.unwrap_or(false),
                value: match c.value.clone() {
                    rovervalidate::service::Value::Number(n) => models::ServicesAuthorServiceVersionGet200ResponseConfigurationInnerValue(RawValue::from_string(format!("{}", n)).unwrap()),
                    rovervalidate::service::Value::String(s) => models::ServicesAuthorServiceVersionGet200ResponseConfigurationInnerValue(RawValue::from_string(format!("\"{}\"", s)).unwrap()),
                },
            })
        }

        Ok(
            ServicesAuthorServiceVersionGetResponse::Status200_AFullDescriptionOfTheServiceAtThisVersion(
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
                    built_at,
                    outputs: service.0.outputs,
                    configuration,
                },
            ),
        )
    }

    /// Build a fully qualified service version.
    /// `RoverState` - This function can run *only when dormant*
    /// TODO: fs_lock
    /// ServicesAuthorServiceVersionPost - POST /services/{author}/{service}/{version}
    async fn services_author_service_version_post(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: ServicesAuthorServiceVersionPostPathParams,
    ) -> Result<ServicesAuthorServiceVersionPostResponse, ()> {
        if let Some(rover_state) = self.try_get_dormant().await {
            let _ = if let Err(e) = self.app.build_service(path_params, rover_state).await {
                warn!("{:#?}", &e);
                match e {
                    Error::BuildLog(build_log) => {
                        return Ok(
                            ServicesAuthorServiceVersionPostResponse::Status400_TheBuildFailed(
                                ServicesAuthorServiceVersionPost400Response {
                                    build_log,
                                    message: "A build error occured".to_string(),
                                },
                            ),
                        );
                    }
                    _ => {
                        return Ok(
                            ServicesAuthorServiceVersionPostResponse::Status400_TheBuildFailed(
                                ServicesAuthorServiceVersionPost400Response {
                                    build_log: vec![],
                                    message: format!("{:?}", e),
                                },
                            ),
                        );
                    }
                }
            };

            Ok(ServicesAuthorServiceVersionPostResponse::Status200_TheServiceWasBuiltSuccessfully)
        } else {
            let msg = "unable to perform request, rover is running";
            warn!(msg);
            return Ok(
                ServicesAuthorServiceVersionPostResponse::Status400_TheBuildFailed(
                    ServicesAuthorServiceVersionPost400Response {
                        build_log: vec![],
                        message: msg.to_string(),
                    },
                ),
            );
        }
    }

    /// Retrieve the list of all authors that have parsable services. With these authors you can query further for services.
    /// `RoverState` - This function can run *always*
    /// TODO: fs_lock
    /// ServicesGet - GET /services
    async fn services_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
    ) -> Result<ServicesGetResponse, ()> {
        let authors = warn_generic!(self.app.get_authors().await, ServicesGetResponse);

        Ok(ServicesGetResponse::Status200_TheListOfAuthors(authors))
    }

    /// Retrieve the list of parsable services for a specific author.
    /// `RoverState` - This function can run *always*
    /// TODO: fs_lock
    /// ServicesAuthorGet - GET /services/{author}
    async fn services_author_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: ServicesAuthorGetPathParams,
    ) -> Result<ServicesAuthorGetResponse, ()> {
        let services = warn_generic!(
            self.app.get_services(path_params).await,
            ServicesAuthorGetResponse
        );

        Ok(ServicesAuthorGetResponse::Status200_TheListOfServicesForTheAuthor(services))
    }
}
