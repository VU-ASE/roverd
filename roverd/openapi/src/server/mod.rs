use std::collections::HashMap;

use axum::{body::Body, extract::*, response::Response, routing::*};
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::{header::CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue, Method, StatusCode};
use tracing::error;
use validator::{Validate, ValidationErrors};

use crate::{header, types::*};

#[allow(unused_imports)]
use crate::{apis, models};

/// Setup API Server.
pub fn new<I, A>(api_impl: I) -> Router
where
    I: AsRef<A> + Clone + Send + Sync + 'static,
    A: apis::health::Health
        + apis::pipeline::Pipeline
        + apis::services::Services
        + apis::sources::Sources
        + 'static,
{
    // build our application with a route
    Router::new()
        .route("/logs/:name", get(logs_name_get::<I, A>))
        .route(
            "/pipeline",
            get(pipeline_get::<I, A>).post(pipeline_post::<I, A>),
        )
        .route("/pipeline/start", post(pipeline_start_post::<I, A>))
        .route("/pipeline/stop", post(pipeline_stop_post::<I, A>))
        .route(
            "/services",
            get(services_get::<I, A>).post(services_post::<I, A>),
        )
        .route("/services/:author", get(services_author_get::<I, A>))
        .route(
            "/services/:author/:service",
            get(services_author_service_get::<I, A>),
        )
        .route(
            "/services/:author/:service/:version",
            delete(services_author_service_version_delete::<I, A>)
                .get(services_author_service_version_get::<I, A>)
                .post(services_author_service_version_post::<I, A>),
        )
        .route(
            "/sources",
            delete(sources_delete::<I, A>)
                .get(sources_get::<I, A>)
                .post(sources_post::<I, A>),
        )
        .route("/status", get(status_get::<I, A>))
        .route("/update", post(update_post::<I, A>))
        .with_state(api_impl)
}

#[tracing::instrument(skip_all)]
fn status_get_validation() -> std::result::Result<(), ValidationErrors> {
    Ok(())
}
/// StatusGet - GET /status
#[tracing::instrument(skip_all)]
async fn status_get<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::health::Health,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || status_get_validation())
        .await
        .unwrap();

    let Ok(()) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl.as_ref().status_get(method, host, cookies).await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::health::StatusGetResponse::Status200_TheHealthAndVersioningInformation(body) => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::health::StatusGetResponse::Status400_AnErrorOccurred(body) => {
                let mut response = response.status(400);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn update_post_validation() -> std::result::Result<(), ValidationErrors> {
    Ok(())
}
/// UpdatePost - POST /update
#[tracing::instrument(skip_all)]
async fn update_post<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::health::Health,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || update_post_validation())
        .await
        .unwrap();

    let Ok(()) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl.as_ref().update_post(method, host, cookies).await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::health::UpdatePostResponse::Status200_TheRoverdDaemonProcessInitiatedASelf(
                body,
            ) => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::health::UpdatePostResponse::Status400_AnErrorOccurred(body) => {
                let mut response = response.status(400);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::health::UpdatePostResponse::Status401_UnauthorizedAccess => {
                let mut response = response.status(401);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn logs_name_get_validation(
    path_params: models::LogsNameGetPathParams,
    query_params: models::LogsNameGetQueryParams,
) -> std::result::Result<
    (
        models::LogsNameGetPathParams,
        models::LogsNameGetQueryParams,
    ),
    ValidationErrors,
> {
    path_params.validate()?;
    query_params.validate()?;

    Ok((path_params, query_params))
}
/// LogsNameGet - GET /logs/{name}
#[tracing::instrument(skip_all)]
async fn logs_name_get<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::LogsNameGetPathParams>,
    Query(query_params): Query<models::LogsNameGetQueryParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::pipeline::Pipeline,
{
    #[allow(clippy::redundant_closure)]
    let validation =
        tokio::task::spawn_blocking(move || logs_name_get_validation(path_params, query_params))
            .await
            .unwrap();

    let Ok((path_params, query_params)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .logs_name_get(method, host, cookies, path_params, query_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::pipeline::LogsNameGetResponse::Status200_TheCollectionOfLogs(body) => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::pipeline::LogsNameGetResponse::Status400_AnErrorOccurred(body) => {
                let mut response = response.status(400);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::pipeline::LogsNameGetResponse::Status404_EntityNotFound => {
                let mut response = response.status(404);
                response.body(Body::empty())
            }
            apis::pipeline::LogsNameGetResponse::Status401_UnauthorizedAccess => {
                let mut response = response.status(401);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn pipeline_get_validation() -> std::result::Result<(), ValidationErrors> {
    Ok(())
}
/// PipelineGet - GET /pipeline
#[tracing::instrument(skip_all)]
async fn pipeline_get<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::pipeline::Pipeline,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || pipeline_get_validation())
        .await
        .unwrap();

    let Ok(()) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl.as_ref().pipeline_get(method, host, cookies).await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::pipeline::PipelineGetResponse::Status200_PipelineStatusAndAnArrayOfProcesses(
                body,
            ) => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::pipeline::PipelineGetResponse::Status400_AnErrorOccurred(body) => {
                let mut response = response.status(400);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::pipeline::PipelineGetResponse::Status401_UnauthorizedAccess => {
                let mut response = response.status(401);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct PipelinePostBodyValidator<'a> {
    #[validate(nested)]
    body: &'a Vec<models::PipelinePostRequestInner>,
}

#[tracing::instrument(skip_all)]
fn pipeline_post_validation(
    body: Vec<models::PipelinePostRequestInner>,
) -> std::result::Result<(Vec<models::PipelinePostRequestInner>,), ValidationErrors> {
    let b = PipelinePostBodyValidator { body: &body };
    b.validate()?;

    Ok((body,))
}
/// PipelinePost - POST /pipeline
#[tracing::instrument(skip_all)]
async fn pipeline_post<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    State(api_impl): State<I>,
    Json(body): Json<Vec<models::PipelinePostRequestInner>>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::pipeline::Pipeline,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || pipeline_post_validation(body))
        .await
        .unwrap();

    let Ok((body,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .pipeline_post(method, host, cookies, body)
        .await;

    let mut response = Response::builder();

    let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::pipeline::PipelinePostResponse::Status200_ThePipelineWasUpdatedSuccessfully
                                                => {
                                                  let mut response = response.status(200);
                                                  response.body(Body::empty())
                                                },
                                                apis::pipeline::PipelinePostResponse::Status400_ThePipelineWasNotValidAndCouldNotBeSet
                                                    (body)
                                                => {
                                                  let mut response = response.status(400);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::pipeline::PipelinePostResponse::Status401_UnauthorizedAccess
                                                => {
                                                  let mut response = response.status(401);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn pipeline_start_post_validation() -> std::result::Result<(), ValidationErrors> {
    Ok(())
}
/// PipelineStartPost - POST /pipeline/start
#[tracing::instrument(skip_all)]
async fn pipeline_start_post<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::pipeline::Pipeline,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || pipeline_start_post_validation())
        .await
        .unwrap();

    let Ok(()) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .pipeline_start_post(method, host, cookies)
        .await;

    let mut response = Response::builder();

    let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::pipeline::PipelineStartPostResponse::Status200_ThePipelineWasStartedSuccessfully
                                                => {
                                                  let mut response = response.status(200);
                                                  response.body(Body::empty())
                                                },
                                                apis::pipeline::PipelineStartPostResponse::Status400_AnErrorOccurred
                                                    (body)
                                                => {
                                                  let mut response = response.status(400);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::pipeline::PipelineStartPostResponse::Status401_UnauthorizedAccess
                                                => {
                                                  let mut response = response.status(401);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn pipeline_stop_post_validation() -> std::result::Result<(), ValidationErrors> {
    Ok(())
}
/// PipelineStopPost - POST /pipeline/stop
#[tracing::instrument(skip_all)]
async fn pipeline_stop_post<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::pipeline::Pipeline,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || pipeline_stop_post_validation())
        .await
        .unwrap();

    let Ok(()) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .pipeline_stop_post(method, host, cookies)
        .await;

    let mut response = Response::builder();

    let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::pipeline::PipelineStopPostResponse::Status200_ThePipelineWasStoppedSuccessfully
                                                => {
                                                  let mut response = response.status(200);
                                                  response.body(Body::empty())
                                                },
                                                apis::pipeline::PipelineStopPostResponse::Status400_AnErrorOccurred
                                                    (body)
                                                => {
                                                  let mut response = response.status(400);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::pipeline::PipelineStopPostResponse::Status401_UnauthorizedAccess
                                                => {
                                                  let mut response = response.status(401);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn services_author_get_validation(
    path_params: models::ServicesAuthorGetPathParams,
) -> std::result::Result<(models::ServicesAuthorGetPathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// ServicesAuthorGet - GET /services/{author}
#[tracing::instrument(skip_all)]
async fn services_author_get<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::ServicesAuthorGetPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::services::Services,
{
    #[allow(clippy::redundant_closure)]
    let validation =
        tokio::task::spawn_blocking(move || services_author_get_validation(path_params))
            .await
            .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .services_author_get(method, host, cookies, path_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::services::ServicesAuthorGetResponse::Status200_TheListOfServicesForTheAuthor(
                body,
            ) => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::services::ServicesAuthorGetResponse::Status400_AnErrorOccurred(body) => {
                let mut response = response.status(400);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::services::ServicesAuthorGetResponse::Status404_EntityNotFound => {
                let mut response = response.status(404);
                response.body(Body::empty())
            }
            apis::services::ServicesAuthorGetResponse::Status401_UnauthorizedAccess => {
                let mut response = response.status(401);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn services_author_service_get_validation(
    path_params: models::ServicesAuthorServiceGetPathParams,
) -> std::result::Result<(models::ServicesAuthorServiceGetPathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// ServicesAuthorServiceGet - GET /services/{author}/{service}
#[tracing::instrument(skip_all)]
async fn services_author_service_get<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::ServicesAuthorServiceGetPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::services::Services,
{
    #[allow(clippy::redundant_closure)]
    let validation =
        tokio::task::spawn_blocking(move || services_author_service_get_validation(path_params))
            .await
            .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .services_author_service_get(method, host, cookies, path_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::services::ServicesAuthorServiceGetResponse::Status200_TheListOfVersionsForThisAuthorAndServiceName
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::services::ServicesAuthorServiceGetResponse::Status400_AnErrorOccurred
                                                    (body)
                                                => {
                                                  let mut response = response.status(400);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::services::ServicesAuthorServiceGetResponse::Status404_EntityNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                                apis::services::ServicesAuthorServiceGetResponse::Status401_UnauthorizedAccess
                                                => {
                                                  let mut response = response.status(401);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn services_author_service_version_delete_validation(
    path_params: models::ServicesAuthorServiceVersionDeletePathParams,
) -> std::result::Result<(models::ServicesAuthorServiceVersionDeletePathParams,), ValidationErrors>
{
    path_params.validate()?;

    Ok((path_params,))
}
/// ServicesAuthorServiceVersionDelete - DELETE /services/{author}/{service}/{version}
#[tracing::instrument(skip_all)]
async fn services_author_service_version_delete<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::ServicesAuthorServiceVersionDeletePathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::services::Services,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || {
        services_author_service_version_delete_validation(path_params)
    })
    .await
    .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .services_author_service_version_delete(method, host, cookies, path_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::services::ServicesAuthorServiceVersionDeleteResponse::Status200_TheServiceVersionWasDeletedSuccessfully
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::services::ServicesAuthorServiceVersionDeleteResponse::Status400_AnErrorOccurred
                                                    (body)
                                                => {
                                                  let mut response = response.status(400);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::services::ServicesAuthorServiceVersionDeleteResponse::Status404_EntityNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                                apis::services::ServicesAuthorServiceVersionDeleteResponse::Status401_UnauthorizedAccess
                                                => {
                                                  let mut response = response.status(401);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn services_author_service_version_get_validation(
    path_params: models::ServicesAuthorServiceVersionGetPathParams,
) -> std::result::Result<(models::ServicesAuthorServiceVersionGetPathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// ServicesAuthorServiceVersionGet - GET /services/{author}/{service}/{version}
#[tracing::instrument(skip_all)]
async fn services_author_service_version_get<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::ServicesAuthorServiceVersionGetPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::services::Services,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || {
        services_author_service_version_get_validation(path_params)
    })
    .await
    .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .services_author_service_version_get(method, host, cookies, path_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::services::ServicesAuthorServiceVersionGetResponse::Status200_TheServiceConfiguration
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::services::ServicesAuthorServiceVersionGetResponse::Status400_AnErrorOccurred
                                                    (body)
                                                => {
                                                  let mut response = response.status(400);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::services::ServicesAuthorServiceVersionGetResponse::Status404_EntityNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                                apis::services::ServicesAuthorServiceVersionGetResponse::Status401_UnauthorizedAccess
                                                => {
                                                  let mut response = response.status(401);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn services_author_service_version_post_validation(
    path_params: models::ServicesAuthorServiceVersionPostPathParams,
) -> std::result::Result<(models::ServicesAuthorServiceVersionPostPathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// ServicesAuthorServiceVersionPost - POST /services/{author}/{service}/{version}
#[tracing::instrument(skip_all)]
async fn services_author_service_version_post<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::ServicesAuthorServiceVersionPostPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::services::Services,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || {
        services_author_service_version_post_validation(path_params)
    })
    .await
    .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .services_author_service_version_post(method, host, cookies, path_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::services::ServicesAuthorServiceVersionPostResponse::Status200_TheServiceWasBuiltSuccessfully
                                                => {
                                                  let mut response = response.status(200);
                                                  response.body(Body::empty())
                                                },
                                                apis::services::ServicesAuthorServiceVersionPostResponse::Status400_TheBuildFailed
                                                    (body)
                                                => {
                                                  let mut response = response.status(400);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::services::ServicesAuthorServiceVersionPostResponse::Status404_EntityNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                                apis::services::ServicesAuthorServiceVersionPostResponse::Status401_UnauthorizedAccess
                                                => {
                                                  let mut response = response.status(401);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn services_get_validation() -> std::result::Result<(), ValidationErrors> {
    Ok(())
}
/// ServicesGet - GET /services
#[tracing::instrument(skip_all)]
async fn services_get<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::services::Services,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || services_get_validation())
        .await
        .unwrap();

    let Ok(()) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl.as_ref().services_get(method, host, cookies).await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::services::ServicesGetResponse::Status200_TheListOfAuthors(body) => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::services::ServicesGetResponse::Status400_AnErrorOccurred(body) => {
                let mut response = response.status(400);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::services::ServicesGetResponse::Status401_UnauthorizedAccess => {
                let mut response = response.status(401);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn services_post_validation() -> std::result::Result<(), ValidationErrors> {
    Ok(())
}
/// ServicesPost - POST /services
#[tracing::instrument(skip_all)]
async fn services_post<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    State(api_impl): State<I>,
    body: Multipart,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::services::Services,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || services_post_validation())
        .await
        .unwrap();

    let Ok(()) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .services_post(method, host, cookies, body)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::services::ServicesPostResponse::Status200_TheServiceWasUploadedSuccessfully(
                body,
            ) => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::services::ServicesPostResponse::Status400_AnErrorOccurred(body) => {
                let mut response = response.status(400);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::services::ServicesPostResponse::Status401_UnauthorizedAccess => {
                let mut response = response.status(401);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct SourcesDeleteBodyValidator<'a> {
    #[validate(nested)]
    body: &'a models::SourcesPostRequest,
}

#[tracing::instrument(skip_all)]
fn sources_delete_validation(
    body: models::SourcesPostRequest,
) -> std::result::Result<(models::SourcesPostRequest,), ValidationErrors> {
    let b = SourcesDeleteBodyValidator { body: &body };
    b.validate()?;

    Ok((body,))
}
/// SourcesDelete - DELETE /sources
#[tracing::instrument(skip_all)]
async fn sources_delete<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    State(api_impl): State<I>,
    Json(body): Json<models::SourcesPostRequest>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::sources::Sources,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || sources_delete_validation(body))
        .await
        .unwrap();

    let Ok((body,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .sources_delete(method, host, cookies, body)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::sources::SourcesDeleteResponse::Status200_TheSourceWasDeletedSuccessfully => {
                let mut response = response.status(200);
                response.body(Body::empty())
            }
            apis::sources::SourcesDeleteResponse::Status400_AnErrorOccurred(body) => {
                let mut response = response.status(400);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::sources::SourcesDeleteResponse::Status404_EntityNotFound => {
                let mut response = response.status(404);
                response.body(Body::empty())
            }
            apis::sources::SourcesDeleteResponse::Status401_UnauthorizedAccess => {
                let mut response = response.status(401);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn sources_get_validation() -> std::result::Result<(), ValidationErrors> {
    Ok(())
}
/// SourcesGet - GET /sources
#[tracing::instrument(skip_all)]
async fn sources_get<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::sources::Sources,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || sources_get_validation())
        .await
        .unwrap();

    let Ok(()) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl.as_ref().sources_get(method, host, cookies).await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::sources::SourcesGetResponse::Status200_AnArrayOfSources(body) => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::sources::SourcesGetResponse::Status400_AnErrorOccurred(body) => {
                let mut response = response.status(400);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::sources::SourcesGetResponse::Status401_UnauthorizedAccess => {
                let mut response = response.status(401);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct SourcesPostBodyValidator<'a> {
    #[validate(nested)]
    body: &'a models::SourcesPostRequest,
}

#[tracing::instrument(skip_all)]
fn sources_post_validation(
    body: models::SourcesPostRequest,
) -> std::result::Result<(models::SourcesPostRequest,), ValidationErrors> {
    let b = SourcesPostBodyValidator { body: &body };
    b.validate()?;

    Ok((body,))
}
/// SourcesPost - POST /sources
#[tracing::instrument(skip_all)]
async fn sources_post<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    State(api_impl): State<I>,
    Json(body): Json<models::SourcesPostRequest>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::sources::Sources,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || sources_post_validation(body))
        .await
        .unwrap();

    let Ok((body,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .sources_post(method, host, cookies, body)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::sources::SourcesPostResponse::Status200_TheSourceWasAddedSuccessfully => {
                let mut response = response.status(200);
                response.body(Body::empty())
            }
            apis::sources::SourcesPostResponse::Status400_AnErrorOccurred(body) => {
                let mut response = response.status(400);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::sources::SourcesPostResponse::Status401_UnauthorizedAccess => {
                let mut response = response.status(401);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}
