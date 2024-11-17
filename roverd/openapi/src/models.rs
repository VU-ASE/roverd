#![allow(unused_qualifications)]

use http::HeaderValue;
use validator::Validate;

#[cfg(feature = "server")]
use crate::header;
use crate::{models, types::*};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PipelineNameGetPathParams {
    /// The name of the service running as a process in the pipeline
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PipelineNameGetQueryParams {
    /// The number of log lines to retrieve
    #[serde(rename = "log_lines")]
    #[validate(range(min = 1, max = 1000))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_lines: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PipelinePostQueryParams {
    /// The action to perform on the pipeline
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "action")]
    pub action: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServicesNameGetPathParams {
    /// The name of the service
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServicesNameVersionDeletePathParams {
    /// The name of the service
    pub name: String,
    /// The version of the service
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServicesNameVersionGetPathParams {
    /// The name of the service
    pub name: String,
    /// The version of the service
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServicesNameVersionPostPathParams {
    /// The name of the service
    pub name: String,
    /// The version of the service
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServicesNameVersionPostQueryParams {
    /// The action to perform on the service version
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "action")]
    pub action: String,
}

/// The status of the roverd process
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum DaemonStatus {
    #[serde(rename = "operational")]
    Operational,
    #[serde(rename = "recoverable")]
    Recoverable,
    #[serde(rename = "unrecoverable")]
    Unrecoverable,
}

impl std::fmt::Display for DaemonStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            DaemonStatus::Operational => write!(f, "operational"),
            DaemonStatus::Recoverable => write!(f, "recoverable"),
            DaemonStatus::Unrecoverable => write!(f, "unrecoverable"),
        }
    }
}

impl std::str::FromStr for DaemonStatus {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "operational" => std::result::Result::Ok(DaemonStatus::Operational),
            "recoverable" => std::result::Result::Ok(DaemonStatus::Recoverable),
            "unrecoverable" => std::result::Result::Ok(DaemonStatus::Unrecoverable),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

/// DuplicateServiceError
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DuplicateServiceError(String);

impl validator::Validate for DuplicateServiceError {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::convert::From<String> for DuplicateServiceError {
    fn from(x: String) -> Self {
        DuplicateServiceError(x)
    }
}

impl std::fmt::Display for DuplicateServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl std::str::FromStr for DuplicateServiceError {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(DuplicateServiceError(x.to_string()))
    }
}

impl std::convert::From<DuplicateServiceError> for String {
    fn from(x: DuplicateServiceError) -> Self {
        x.0
    }
}

impl std::ops::Deref for DuplicateServiceError {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for DuplicateServiceError {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GenericError {
    /// A message describing the error
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// A code describing the error (this is not an HTTP status code)
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
}

impl GenericError {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> GenericError {
        GenericError {
            message: None,
            code: None,
        }
    }
}

/// Converts the GenericError value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for GenericError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            self.message
                .as_ref()
                .map(|message| ["message".to_string(), message.to_string()].join(",")),
            self.code
                .as_ref()
                .map(|code| ["code".to_string(), code.to_string()].join(",")),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GenericError value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GenericError {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub message: Vec<String>,
            pub code: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing GenericError".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing GenericError".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GenericError {
            message: intermediate_rep.message.into_iter().next(),
            code: intermediate_rep.code.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GenericError> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<GenericError>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<GenericError>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for GenericError - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<GenericError> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <GenericError as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into GenericError - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PipelineGet200Response {
    #[serde(rename = "pipeline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<models::PipelineGet200ResponsePipeline>,

    #[serde(rename = "processes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processes: Option<Vec<models::PipelineGet200ResponseProcessesInner>>,
}

impl PipelineGet200Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> PipelineGet200Response {
        PipelineGet200Response {
            pipeline: None,
            processes: None,
        }
    }
}

/// Converts the PipelineGet200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for PipelineGet200Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping pipeline in query parameter serialization

            // Skipping processes in query parameter serialization

        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PipelineGet200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PipelineGet200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub pipeline: Vec<models::PipelineGet200ResponsePipeline>,
            pub processes: Vec<Vec<models::PipelineGet200ResponseProcessesInner>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing PipelineGet200Response".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "pipeline" => intermediate_rep.pipeline.push(<models::PipelineGet200ResponsePipeline as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "processes" => return std::result::Result::Err("Parsing a container in this style is not supported in PipelineGet200Response".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing PipelineGet200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PipelineGet200Response {
            pipeline: intermediate_rep.pipeline.into_iter().next(),
            processes: intermediate_rep.processes.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PipelineGet200Response> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<PipelineGet200Response>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<PipelineGet200Response>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for PipelineGet200Response - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<PipelineGet200Response> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <PipelineGet200Response as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into PipelineGet200Response - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PipelineGet200ResponsePipeline {
    #[serde(rename = "status")]
    pub status: models::PipelineStatus,

    /// Milliseconds since epoch when the pipeline was manually started
    #[serde(rename = "last_start")]
    pub last_start: i64,

    /// Milliseconds since epoch when the pipeline was manually stopped
    #[serde(rename = "last_stop")]
    pub last_stop: i64,

    /// Milliseconds since epoch when the pipeline was automatically restarted (on process faults)
    #[serde(rename = "last_restart")]
    pub last_restart: i64,

    #[serde(rename = "validation_errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<models::PipelineGet200ResponsePipelineValidationErrors>,
}

impl PipelineGet200ResponsePipeline {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        status: models::PipelineStatus,
        last_start: i64,
        last_stop: i64,
        last_restart: i64,
    ) -> PipelineGet200ResponsePipeline {
        PipelineGet200ResponsePipeline {
            status,
            last_start,
            last_stop,
            last_restart,
            validation_errors: None,
        }
    }
}

/// Converts the PipelineGet200ResponsePipeline value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for PipelineGet200ResponsePipeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping status in query parameter serialization
            Some("last_start".to_string()),
            Some(self.last_start.to_string()),
            Some("last_stop".to_string()),
            Some(self.last_stop.to_string()),
            Some("last_restart".to_string()),
            Some(self.last_restart.to_string()),
            // Skipping validation_errors in query parameter serialization
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PipelineGet200ResponsePipeline value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PipelineGet200ResponsePipeline {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub status: Vec<models::PipelineStatus>,
            pub last_start: Vec<i64>,
            pub last_stop: Vec<i64>,
            pub last_restart: Vec<i64>,
            pub validation_errors: Vec<models::PipelineGet200ResponsePipelineValidationErrors>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing PipelineGet200ResponsePipeline".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<models::PipelineStatus as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "last_start" => intermediate_rep.last_start.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "last_stop" => intermediate_rep.last_stop.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "last_restart" => intermediate_rep.last_restart.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "validation_errors" => intermediate_rep.validation_errors.push(<models::PipelineGet200ResponsePipelineValidationErrors as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing PipelineGet200ResponsePipeline".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PipelineGet200ResponsePipeline {
            status: intermediate_rep
                .status
                .into_iter()
                .next()
                .ok_or_else(|| "status missing in PipelineGet200ResponsePipeline".to_string())?,
            last_start: intermediate_rep
                .last_start
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "last_start missing in PipelineGet200ResponsePipeline".to_string()
                })?,
            last_stop: intermediate_rep
                .last_stop
                .into_iter()
                .next()
                .ok_or_else(|| "last_stop missing in PipelineGet200ResponsePipeline".to_string())?,
            last_restart: intermediate_rep
                .last_restart
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "last_restart missing in PipelineGet200ResponsePipeline".to_string()
                })?,
            validation_errors: intermediate_rep.validation_errors.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PipelineGet200ResponsePipeline> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<PipelineGet200ResponsePipeline>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<PipelineGet200ResponsePipeline>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for PipelineGet200ResponsePipeline - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<PipelineGet200ResponsePipeline>
{
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PipelineGet200ResponsePipeline as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PipelineGet200ResponsePipeline - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

/// If status is invalid, this array shows the validation errors

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PipelineGet200ResponsePipelineValidationErrors {
    #[serde(rename = "unmet_streams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmet_streams: Option<Vec<models::UnmetStreamError>>,

    #[serde(rename = "unmet_services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmet_services: Option<Vec<models::UnmetServiceError>>,

    #[serde(rename = "duplicate_service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_service: Option<Vec<models::DuplicateServiceError>>,
}

impl PipelineGet200ResponsePipelineValidationErrors {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> PipelineGet200ResponsePipelineValidationErrors {
        PipelineGet200ResponsePipelineValidationErrors {
            unmet_streams: None,
            unmet_services: None,
            duplicate_service: None,
        }
    }
}

/// Converts the PipelineGet200ResponsePipelineValidationErrors value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for PipelineGet200ResponsePipelineValidationErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping unmet_streams in query parameter serialization

            // Skipping unmet_services in query parameter serialization
            self.duplicate_service.as_ref().map(|duplicate_service| {
                [
                    "duplicate_service".to_string(),
                    duplicate_service
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                ]
                .join(",")
            }),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PipelineGet200ResponsePipelineValidationErrors value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PipelineGet200ResponsePipelineValidationErrors {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub unmet_streams: Vec<Vec<models::UnmetStreamError>>,
            pub unmet_services: Vec<Vec<models::UnmetServiceError>>,
            pub duplicate_service: Vec<Vec<models::DuplicateServiceError>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err(
                    "Missing value while parsing PipelineGet200ResponsePipelineValidationErrors"
                        .to_string(),
                ),
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "unmet_streams" => return std::result::Result::Err("Parsing a container in this style is not supported in PipelineGet200ResponsePipelineValidationErrors".to_string()),
                    "unmet_services" => return std::result::Result::Err("Parsing a container in this style is not supported in PipelineGet200ResponsePipelineValidationErrors".to_string()),
                    "duplicate_service" => return std::result::Result::Err("Parsing a container in this style is not supported in PipelineGet200ResponsePipelineValidationErrors".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing PipelineGet200ResponsePipelineValidationErrors".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PipelineGet200ResponsePipelineValidationErrors {
            unmet_streams: intermediate_rep.unmet_streams.into_iter().next(),
            unmet_services: intermediate_rep.unmet_services.into_iter().next(),
            duplicate_service: intermediate_rep.duplicate_service.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PipelineGet200ResponsePipelineValidationErrors> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<PipelineGet200ResponsePipelineValidationErrors>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<PipelineGet200ResponsePipelineValidationErrors>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PipelineGet200ResponsePipelineValidationErrors - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<PipelineGet200ResponsePipelineValidationErrors>
{
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PipelineGet200ResponsePipelineValidationErrors as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PipelineGet200ResponsePipelineValidationErrors - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PipelineGet200ResponseProcessesInner {
    /// The name of the service running as a process
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "status")]
    pub status: models::ProcessStatus,

    /// The process ID
    #[serde(rename = "pid")]
    pub pid: i32,

    /// The number of milliseconds the process has been running
    #[serde(rename = "uptime")]
    pub uptime: i64,

    /// The amount of memory used by the process in megabytes
    #[serde(rename = "memory")]
    pub memory: i32,

    /// The percentage of CPU used by the process
    #[serde(rename = "cpu")]
    pub cpu: i32,

    /// The number of faults that have occurred (causing the pipeline to restart) since last_start
    #[serde(rename = "faults")]
    pub faults: i32,
}

impl PipelineGet200ResponseProcessesInner {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        name: String,
        status: models::ProcessStatus,
        pid: i32,
        uptime: i64,
        memory: i32,
        cpu: i32,
        faults: i32,
    ) -> PipelineGet200ResponseProcessesInner {
        PipelineGet200ResponseProcessesInner {
            name,
            status,
            pid,
            uptime,
            memory,
            cpu,
            faults,
        }
    }
}

/// Converts the PipelineGet200ResponseProcessesInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for PipelineGet200ResponseProcessesInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("name".to_string()),
            Some(self.name.to_string()),
            // Skipping status in query parameter serialization
            Some("pid".to_string()),
            Some(self.pid.to_string()),
            Some("uptime".to_string()),
            Some(self.uptime.to_string()),
            Some("memory".to_string()),
            Some(self.memory.to_string()),
            Some("cpu".to_string()),
            Some(self.cpu.to_string()),
            Some("faults".to_string()),
            Some(self.faults.to_string()),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PipelineGet200ResponseProcessesInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PipelineGet200ResponseProcessesInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub status: Vec<models::ProcessStatus>,
            pub pid: Vec<i32>,
            pub uptime: Vec<i64>,
            pub memory: Vec<i32>,
            pub cpu: Vec<i32>,
            pub faults: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing PipelineGet200ResponseProcessesInner"
                            .to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(
                        <models::ProcessStatus as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "pid" => intermediate_rep.pid.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "uptime" => intermediate_rep.uptime.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "memory" => intermediate_rep.memory.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "cpu" => intermediate_rep.cpu.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "faults" => intermediate_rep.faults.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing PipelineGet200ResponseProcessesInner"
                                .to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PipelineGet200ResponseProcessesInner {
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| {
                "name missing in PipelineGet200ResponseProcessesInner".to_string()
            })?,
            status: intermediate_rep.status.into_iter().next().ok_or_else(|| {
                "status missing in PipelineGet200ResponseProcessesInner".to_string()
            })?,
            pid: intermediate_rep
                .pid
                .into_iter()
                .next()
                .ok_or_else(|| "pid missing in PipelineGet200ResponseProcessesInner".to_string())?,
            uptime: intermediate_rep.uptime.into_iter().next().ok_or_else(|| {
                "uptime missing in PipelineGet200ResponseProcessesInner".to_string()
            })?,
            memory: intermediate_rep.memory.into_iter().next().ok_or_else(|| {
                "memory missing in PipelineGet200ResponseProcessesInner".to_string()
            })?,
            cpu: intermediate_rep
                .cpu
                .into_iter()
                .next()
                .ok_or_else(|| "cpu missing in PipelineGet200ResponseProcessesInner".to_string())?,
            faults: intermediate_rep.faults.into_iter().next().ok_or_else(|| {
                "faults missing in PipelineGet200ResponseProcessesInner".to_string()
            })?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PipelineGet200ResponseProcessesInner> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<PipelineGet200ResponseProcessesInner>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<PipelineGet200ResponseProcessesInner>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PipelineGet200ResponseProcessesInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<PipelineGet200ResponseProcessesInner>
{
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PipelineGet200ResponseProcessesInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PipelineGet200ResponseProcessesInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PipelineNameGet200Response {
    /// The name of the service running as a process
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "status")]
    pub status: models::ProcessStatus,

    /// The process ID
    #[serde(rename = "pid")]
    pub pid: i32,

    /// The number of milliseconds the process has been running
    #[serde(rename = "uptime")]
    pub uptime: i64,

    /// The amount of memory used by the process in megabytes
    #[serde(rename = "memory")]
    pub memory: i32,

    /// The percentage of CPU used by the process
    #[serde(rename = "cpu")]
    pub cpu: i32,

    /// The number of faults that have occurred (causing the pipeline to restart) since last_start
    #[serde(rename = "faults")]
    pub faults: i32,

    /// The name of the service that this process is running
    #[serde(rename = "service_name")]
    pub service_name: String,

    /// The version of the service that this process is running
    #[serde(rename = "service_version")]
    pub service_version: String,

    /// The latest <log_lines> log lines of the process
    #[serde(rename = "logs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<Vec<String>>,
}

impl PipelineNameGet200Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        name: String,
        status: models::ProcessStatus,
        pid: i32,
        uptime: i64,
        memory: i32,
        cpu: i32,
        faults: i32,
        service_name: String,
        service_version: String,
    ) -> PipelineNameGet200Response {
        PipelineNameGet200Response {
            name,
            status,
            pid,
            uptime,
            memory,
            cpu,
            faults,
            service_name,
            service_version,
            logs: None,
        }
    }
}

/// Converts the PipelineNameGet200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for PipelineNameGet200Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("name".to_string()),
            Some(self.name.to_string()),
            // Skipping status in query parameter serialization
            Some("pid".to_string()),
            Some(self.pid.to_string()),
            Some("uptime".to_string()),
            Some(self.uptime.to_string()),
            Some("memory".to_string()),
            Some(self.memory.to_string()),
            Some("cpu".to_string()),
            Some(self.cpu.to_string()),
            Some("faults".to_string()),
            Some(self.faults.to_string()),
            Some("service_name".to_string()),
            Some(self.service_name.to_string()),
            Some("service_version".to_string()),
            Some(self.service_version.to_string()),
            self.logs.as_ref().map(|logs| {
                [
                    "logs".to_string(),
                    logs.iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                ]
                .join(",")
            }),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PipelineNameGet200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PipelineNameGet200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub status: Vec<models::ProcessStatus>,
            pub pid: Vec<i32>,
            pub uptime: Vec<i64>,
            pub memory: Vec<i32>,
            pub cpu: Vec<i32>,
            pub faults: Vec<i32>,
            pub service_name: Vec<String>,
            pub service_version: Vec<String>,
            pub logs: Vec<Vec<String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing PipelineNameGet200Response".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<models::ProcessStatus as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "pid" => intermediate_rep.pid.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "uptime" => intermediate_rep.uptime.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "memory" => intermediate_rep.memory.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "cpu" => intermediate_rep.cpu.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "faults" => intermediate_rep.faults.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "service_name" => intermediate_rep.service_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "service_version" => intermediate_rep.service_version.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "logs" => return std::result::Result::Err("Parsing a container in this style is not supported in PipelineNameGet200Response".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing PipelineNameGet200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PipelineNameGet200Response {
            name: intermediate_rep
                .name
                .into_iter()
                .next()
                .ok_or_else(|| "name missing in PipelineNameGet200Response".to_string())?,
            status: intermediate_rep
                .status
                .into_iter()
                .next()
                .ok_or_else(|| "status missing in PipelineNameGet200Response".to_string())?,
            pid: intermediate_rep
                .pid
                .into_iter()
                .next()
                .ok_or_else(|| "pid missing in PipelineNameGet200Response".to_string())?,
            uptime: intermediate_rep
                .uptime
                .into_iter()
                .next()
                .ok_or_else(|| "uptime missing in PipelineNameGet200Response".to_string())?,
            memory: intermediate_rep
                .memory
                .into_iter()
                .next()
                .ok_or_else(|| "memory missing in PipelineNameGet200Response".to_string())?,
            cpu: intermediate_rep
                .cpu
                .into_iter()
                .next()
                .ok_or_else(|| "cpu missing in PipelineNameGet200Response".to_string())?,
            faults: intermediate_rep
                .faults
                .into_iter()
                .next()
                .ok_or_else(|| "faults missing in PipelineNameGet200Response".to_string())?,
            service_name: intermediate_rep
                .service_name
                .into_iter()
                .next()
                .ok_or_else(|| "service_name missing in PipelineNameGet200Response".to_string())?,
            service_version: intermediate_rep
                .service_version
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "service_version missing in PipelineNameGet200Response".to_string()
                })?,
            logs: intermediate_rep.logs.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PipelineNameGet200Response> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<PipelineNameGet200Response>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<PipelineNameGet200Response>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for PipelineNameGet200Response - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<PipelineNameGet200Response> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <PipelineNameGet200Response as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into PipelineNameGet200Response - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

/// The status of the entire pipeline corresponding to a state machine
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum PipelineStatus {
    #[serde(rename = "started")]
    Started,
    #[serde(rename = "restarting")]
    Restarting,
    #[serde(rename = "invalid")]
    Invalid,
    #[serde(rename = "valid")]
    Valid,
}

impl std::fmt::Display for PipelineStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PipelineStatus::Started => write!(f, "started"),
            PipelineStatus::Restarting => write!(f, "restarting"),
            PipelineStatus::Invalid => write!(f, "invalid"),
            PipelineStatus::Valid => write!(f, "valid"),
        }
    }
}

impl std::str::FromStr for PipelineStatus {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "started" => std::result::Result::Ok(PipelineStatus::Started),
            "restarting" => std::result::Result::Ok(PipelineStatus::Restarting),
            "invalid" => std::result::Result::Ok(PipelineStatus::Invalid),
            "valid" => std::result::Result::Ok(PipelineStatus::Valid),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

/// The status of a process in the pipeline
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum ProcessStatus {
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "stopped")]
    Stopped,
    #[serde(rename = "terminated")]
    Terminated,
    #[serde(rename = "killed")]
    Killed,
}

impl std::fmt::Display for ProcessStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ProcessStatus::Running => write!(f, "running"),
            ProcessStatus::Stopped => write!(f, "stopped"),
            ProcessStatus::Terminated => write!(f, "terminated"),
            ProcessStatus::Killed => write!(f, "killed"),
        }
    }
}

impl std::str::FromStr for ProcessStatus {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "running" => std::result::Result::Ok(ProcessStatus::Running),
            "stopped" => std::result::Result::Ok(ProcessStatus::Stopped),
            "terminated" => std::result::Result::Ok(ProcessStatus::Terminated),
            "killed" => std::result::Result::Ok(ProcessStatus::Killed),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

/// The status of any given service is either enabled or disabled
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum ServiceStatus {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}

impl std::fmt::Display for ServiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ServiceStatus::Enabled => write!(f, "enabled"),
            ServiceStatus::Disabled => write!(f, "disabled"),
        }
    }
}

impl std::str::FromStr for ServiceStatus {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "enabled" => std::result::Result::Ok(ServiceStatus::Enabled),
            "disabled" => std::result::Result::Ok(ServiceStatus::Disabled),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServicesGet200ResponseInner {
    /// The name of the service
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "status")]
    pub status: models::ServiceStatus,

    /// The version that is enabled for this service (if any)
    #[serde(rename = "enabled_version")]
    pub enabled_version: String,
}

impl ServicesGet200ResponseInner {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        name: String,
        status: models::ServiceStatus,
        enabled_version: String,
    ) -> ServicesGet200ResponseInner {
        ServicesGet200ResponseInner {
            name,
            status,
            enabled_version,
        }
    }
}

/// Converts the ServicesGet200ResponseInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ServicesGet200ResponseInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("name".to_string()),
            Some(self.name.to_string()),
            // Skipping status in query parameter serialization
            Some("enabled_version".to_string()),
            Some(self.enabled_version.to_string()),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ServicesGet200ResponseInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ServicesGet200ResponseInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub status: Vec<models::ServiceStatus>,
            pub enabled_version: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ServicesGet200ResponseInner".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(
                        <models::ServiceStatus as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "enabled_version" => intermediate_rep.enabled_version.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing ServicesGet200ResponseInner".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ServicesGet200ResponseInner {
            name: intermediate_rep
                .name
                .into_iter()
                .next()
                .ok_or_else(|| "name missing in ServicesGet200ResponseInner".to_string())?,
            status: intermediate_rep
                .status
                .into_iter()
                .next()
                .ok_or_else(|| "status missing in ServicesGet200ResponseInner".to_string())?,
            enabled_version: intermediate_rep
                .enabled_version
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "enabled_version missing in ServicesGet200ResponseInner".to_string()
                })?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ServicesGet200ResponseInner> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ServicesGet200ResponseInner>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ServicesGet200ResponseInner>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for ServicesGet200ResponseInner - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ServicesGet200ResponseInner> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ServicesGet200ResponseInner as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into ServicesGet200ResponseInner - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServicesNameGet200Response {
    /// The name of the service
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "status")]
    pub status: models::ServiceStatus,

    #[serde(rename = "versions")]
    pub versions: Vec<String>,

    /// The version that is enabled for this service (if any)
    #[serde(rename = "enabled_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_version: Option<String>,
}

impl ServicesNameGet200Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        name: String,
        status: models::ServiceStatus,
        versions: Vec<String>,
    ) -> ServicesNameGet200Response {
        ServicesNameGet200Response {
            name,
            status,
            versions,
            enabled_version: None,
        }
    }
}

/// Converts the ServicesNameGet200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ServicesNameGet200Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("name".to_string()),
            Some(self.name.to_string()),
            // Skipping status in query parameter serialization
            Some("versions".to_string()),
            Some(
                self.versions
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            ),
            self.enabled_version.as_ref().map(|enabled_version| {
                ["enabled_version".to_string(), enabled_version.to_string()].join(",")
            }),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ServicesNameGet200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ServicesNameGet200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub status: Vec<models::ServiceStatus>,
            pub versions: Vec<Vec<String>>,
            pub enabled_version: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ServicesNameGet200Response".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<models::ServiceStatus as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "versions" => return std::result::Result::Err("Parsing a container in this style is not supported in ServicesNameGet200Response".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "enabled_version" => intermediate_rep.enabled_version.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ServicesNameGet200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ServicesNameGet200Response {
            name: intermediate_rep
                .name
                .into_iter()
                .next()
                .ok_or_else(|| "name missing in ServicesNameGet200Response".to_string())?,
            status: intermediate_rep
                .status
                .into_iter()
                .next()
                .ok_or_else(|| "status missing in ServicesNameGet200Response".to_string())?,
            versions: intermediate_rep
                .versions
                .into_iter()
                .next()
                .ok_or_else(|| "versions missing in ServicesNameGet200Response".to_string())?,
            enabled_version: intermediate_rep.enabled_version.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ServicesNameGet200Response> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ServicesNameGet200Response>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ServicesNameGet200Response>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for ServicesNameGet200Response - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ServicesNameGet200Response> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ServicesNameGet200Response as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into ServicesNameGet200Response - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServicesNameVersionGet200Response {
    /// The name of the service
    #[serde(rename = "name")]
    pub name: String,

    /// The version of the service
    #[serde(rename = "version")]
    pub version: String,

    #[serde(rename = "status")]
    pub status: models::ServiceStatus,

    /// The time this version was last built as milliseconds since epoch
    #[serde(rename = "built_at")]
    pub built_at: i64,

    /// The author of the service
    #[serde(rename = "author")]
    pub author: String,

    /// The dependencies/inputs of this service version
    #[serde(rename = "inputs")]
    pub inputs: Vec<models::ServicesNameVersionGet200ResponseInputsInner>,

    /// The output streams of this service version
    #[serde(rename = "outputs")]
    pub outputs: Vec<String>,

    /// The validation errors of this service version (one error per line)
    #[serde(rename = "errors")]
    pub errors: Vec<String>,
}

impl ServicesNameVersionGet200Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        name: String,
        version: String,
        status: models::ServiceStatus,
        built_at: i64,
        author: String,
        inputs: Vec<models::ServicesNameVersionGet200ResponseInputsInner>,
        outputs: Vec<String>,
        errors: Vec<String>,
    ) -> ServicesNameVersionGet200Response {
        ServicesNameVersionGet200Response {
            name,
            version,
            status,
            built_at,
            author,
            inputs,
            outputs,
            errors,
        }
    }
}

/// Converts the ServicesNameVersionGet200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ServicesNameVersionGet200Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("name".to_string()),
            Some(self.name.to_string()),
            Some("version".to_string()),
            Some(self.version.to_string()),
            // Skipping status in query parameter serialization
            Some("built_at".to_string()),
            Some(self.built_at.to_string()),
            Some("author".to_string()),
            Some(self.author.to_string()),
            // Skipping inputs in query parameter serialization
            Some("outputs".to_string()),
            Some(
                self.outputs
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            ),
            Some("errors".to_string()),
            Some(
                self.errors
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            ),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ServicesNameVersionGet200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ServicesNameVersionGet200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub version: Vec<String>,
            pub status: Vec<models::ServiceStatus>,
            pub built_at: Vec<i64>,
            pub author: Vec<String>,
            pub inputs: Vec<Vec<models::ServicesNameVersionGet200ResponseInputsInner>>,
            pub outputs: Vec<Vec<String>>,
            pub errors: Vec<Vec<String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ServicesNameVersionGet200Response".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "version" => intermediate_rep.version.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<models::ServiceStatus as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "built_at" => intermediate_rep.built_at.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "author" => intermediate_rep.author.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "inputs" => return std::result::Result::Err("Parsing a container in this style is not supported in ServicesNameVersionGet200Response".to_string()),
                    "outputs" => return std::result::Result::Err("Parsing a container in this style is not supported in ServicesNameVersionGet200Response".to_string()),
                    "errors" => return std::result::Result::Err("Parsing a container in this style is not supported in ServicesNameVersionGet200Response".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ServicesNameVersionGet200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ServicesNameVersionGet200Response {
            name: intermediate_rep
                .name
                .into_iter()
                .next()
                .ok_or_else(|| "name missing in ServicesNameVersionGet200Response".to_string())?,
            version: intermediate_rep.version.into_iter().next().ok_or_else(|| {
                "version missing in ServicesNameVersionGet200Response".to_string()
            })?,
            status: intermediate_rep
                .status
                .into_iter()
                .next()
                .ok_or_else(|| "status missing in ServicesNameVersionGet200Response".to_string())?,
            built_at: intermediate_rep
                .built_at
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "built_at missing in ServicesNameVersionGet200Response".to_string()
                })?,
            author: intermediate_rep
                .author
                .into_iter()
                .next()
                .ok_or_else(|| "author missing in ServicesNameVersionGet200Response".to_string())?,
            inputs: intermediate_rep
                .inputs
                .into_iter()
                .next()
                .ok_or_else(|| "inputs missing in ServicesNameVersionGet200Response".to_string())?,
            outputs: intermediate_rep.outputs.into_iter().next().ok_or_else(|| {
                "outputs missing in ServicesNameVersionGet200Response".to_string()
            })?,
            errors: intermediate_rep
                .errors
                .into_iter()
                .next()
                .ok_or_else(|| "errors missing in ServicesNameVersionGet200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ServicesNameVersionGet200Response> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ServicesNameVersionGet200Response>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ServicesNameVersionGet200Response>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ServicesNameVersionGet200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<ServicesNameVersionGet200Response>
{
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ServicesNameVersionGet200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ServicesNameVersionGet200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServicesNameVersionGet200ResponseInputsInner {
    /// The name of the service dependency
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,

    /// The streams of the service dependency
    #[serde(rename = "streams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streams: Option<Vec<String>>,
}

impl ServicesNameVersionGet200ResponseInputsInner {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> ServicesNameVersionGet200ResponseInputsInner {
        ServicesNameVersionGet200ResponseInputsInner {
            service: None,
            streams: None,
        }
    }
}

/// Converts the ServicesNameVersionGet200ResponseInputsInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ServicesNameVersionGet200ResponseInputsInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            self.service
                .as_ref()
                .map(|service| ["service".to_string(), service.to_string()].join(",")),
            self.streams.as_ref().map(|streams| {
                [
                    "streams".to_string(),
                    streams
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                ]
                .join(",")
            }),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ServicesNameVersionGet200ResponseInputsInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ServicesNameVersionGet200ResponseInputsInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub service: Vec<String>,
            pub streams: Vec<Vec<String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val =
                match string_iter.next() {
                    Some(x) => x,
                    None => return std::result::Result::Err(
                        "Missing value while parsing ServicesNameVersionGet200ResponseInputsInner"
                            .to_string(),
                    ),
                };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "service" => intermediate_rep.service.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "streams" => return std::result::Result::Err("Parsing a container in this style is not supported in ServicesNameVersionGet200ResponseInputsInner".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ServicesNameVersionGet200ResponseInputsInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ServicesNameVersionGet200ResponseInputsInner {
            service: intermediate_rep.service.into_iter().next(),
            streams: intermediate_rep.streams.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ServicesNameVersionGet200ResponseInputsInner> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ServicesNameVersionGet200ResponseInputsInner>>
    for HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ServicesNameVersionGet200ResponseInputsInner>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ServicesNameVersionGet200ResponseInputsInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue>
    for header::IntoHeaderValue<ServicesNameVersionGet200ResponseInputsInner>
{
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ServicesNameVersionGet200ResponseInputsInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ServicesNameVersionGet200ResponseInputsInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ServicesPost200Response {
    /// The name of the service
    #[serde(rename = "name")]
    pub name: String,

    /// The version of the service
    #[serde(rename = "version")]
    pub version: String,
}

impl ServicesPost200Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(name: String, version: String) -> ServicesPost200Response {
        ServicesPost200Response { name, version }
    }
}

/// Converts the ServicesPost200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ServicesPost200Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("name".to_string()),
            Some(self.name.to_string()),
            Some("version".to_string()),
            Some(self.version.to_string()),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ServicesPost200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ServicesPost200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub version: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ServicesPost200Response".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "version" => intermediate_rep.version.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing ServicesPost200Response".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ServicesPost200Response {
            name: intermediate_rep
                .name
                .into_iter()
                .next()
                .ok_or_else(|| "name missing in ServicesPost200Response".to_string())?,
            version: intermediate_rep
                .version
                .into_iter()
                .next()
                .ok_or_else(|| "version missing in ServicesPost200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ServicesPost200Response> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ServicesPost200Response>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ServicesPost200Response>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for ServicesPost200Response - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ServicesPost200Response> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ServicesPost200Response as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into ServicesPost200Response - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SourcesGet200ResponseInner {
    /// The name of the source
    #[serde(rename = "name")]
    pub name: String,

    /// The URL of the source (without scheme)
    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "version")]
    pub version: String,

    /// The SHA256 hash of the source download, computed over the ZIP file downloaded
    #[serde(rename = "sha")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha: Option<String>,
}

impl SourcesGet200ResponseInner {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(name: String, url: String, version: String) -> SourcesGet200ResponseInner {
        SourcesGet200ResponseInner {
            name,
            url,
            version,
            sha: None,
        }
    }
}

/// Converts the SourcesGet200ResponseInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for SourcesGet200ResponseInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("name".to_string()),
            Some(self.name.to_string()),
            Some("url".to_string()),
            Some(self.url.to_string()),
            Some("version".to_string()),
            Some(self.version.to_string()),
            self.sha
                .as_ref()
                .map(|sha| ["sha".to_string(), sha.to_string()].join(",")),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SourcesGet200ResponseInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SourcesGet200ResponseInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub url: Vec<String>,
            pub version: Vec<String>,
            pub sha: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing SourcesGet200ResponseInner".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "url" => intermediate_rep.url.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "version" => intermediate_rep.version.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "sha" => intermediate_rep.sha.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing SourcesGet200ResponseInner".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SourcesGet200ResponseInner {
            name: intermediate_rep
                .name
                .into_iter()
                .next()
                .ok_or_else(|| "name missing in SourcesGet200ResponseInner".to_string())?,
            url: intermediate_rep
                .url
                .into_iter()
                .next()
                .ok_or_else(|| "url missing in SourcesGet200ResponseInner".to_string())?,
            version: intermediate_rep
                .version
                .into_iter()
                .next()
                .ok_or_else(|| "version missing in SourcesGet200ResponseInner".to_string())?,
            sha: intermediate_rep.sha.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SourcesGet200ResponseInner> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<SourcesGet200ResponseInner>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<SourcesGet200ResponseInner>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for SourcesGet200ResponseInner - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<SourcesGet200ResponseInner> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <SourcesGet200ResponseInner as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into SourcesGet200ResponseInner - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SourcesPostRequest {
    /// The name of the source
    #[serde(rename = "name")]
    pub name: String,

    /// The URL of the source (without scheme)
    #[serde(rename = "url")]
    pub url: String,

    /// The version of the source
    #[serde(rename = "version")]
    pub version: String,
}

impl SourcesPostRequest {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(name: String, url: String, version: String) -> SourcesPostRequest {
        SourcesPostRequest { name, url, version }
    }
}

/// Converts the SourcesPostRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for SourcesPostRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("name".to_string()),
            Some(self.name.to_string()),
            Some("url".to_string()),
            Some(self.url.to_string()),
            Some("version".to_string()),
            Some(self.version.to_string()),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SourcesPostRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SourcesPostRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub url: Vec<String>,
            pub version: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing SourcesPostRequest".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "url" => intermediate_rep.url.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "version" => intermediate_rep.version.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing SourcesPostRequest".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SourcesPostRequest {
            name: intermediate_rep
                .name
                .into_iter()
                .next()
                .ok_or_else(|| "name missing in SourcesPostRequest".to_string())?,
            url: intermediate_rep
                .url
                .into_iter()
                .next()
                .ok_or_else(|| "url missing in SourcesPostRequest".to_string())?,
            version: intermediate_rep
                .version
                .into_iter()
                .next()
                .ok_or_else(|| "version missing in SourcesPostRequest".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SourcesPostRequest> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<SourcesPostRequest>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<SourcesPostRequest>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for SourcesPostRequest - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<SourcesPostRequest> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <SourcesPostRequest as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into SourcesPostRequest - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct StatusGet200Response {
    #[serde(rename = "status")]
    pub status: models::DaemonStatus,

    /// Error message of the daemon status
    #[serde(rename = "error_message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,

    /// The version of the roverd daemon
    #[serde(rename = "version")]
    pub version: String,

    /// The number of milliseconds the roverd daemon process has been running
    #[serde(rename = "uptime")]
    pub uptime: i64,

    /// The operating system of the rover
    #[serde(rename = "os")]
    pub os: String,

    /// The system time of the rover as milliseconds since epoch
    #[serde(rename = "systime")]
    pub systime: i64,

    /// The unique identifier of the rover
    #[serde(rename = "rover_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rover_id: Option<i32>,

    /// The unique name of the rover
    #[serde(rename = "rover_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rover_name: Option<String>,
}

impl StatusGet200Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        status: models::DaemonStatus,
        version: String,
        uptime: i64,
        os: String,
        systime: i64,
    ) -> StatusGet200Response {
        StatusGet200Response {
            status,
            error_message: None,
            version,
            uptime,
            os,
            systime,
            rover_id: None,
            rover_name: None,
        }
    }
}

/// Converts the StatusGet200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for StatusGet200Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping status in query parameter serialization
            self.error_message.as_ref().map(|error_message| {
                ["error_message".to_string(), error_message.to_string()].join(",")
            }),
            Some("version".to_string()),
            Some(self.version.to_string()),
            Some("uptime".to_string()),
            Some(self.uptime.to_string()),
            Some("os".to_string()),
            Some(self.os.to_string()),
            Some("systime".to_string()),
            Some(self.systime.to_string()),
            self.rover_id
                .as_ref()
                .map(|rover_id| ["rover_id".to_string(), rover_id.to_string()].join(",")),
            self.rover_name
                .as_ref()
                .map(|rover_name| ["rover_name".to_string(), rover_name.to_string()].join(",")),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a StatusGet200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for StatusGet200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub status: Vec<models::DaemonStatus>,
            pub error_message: Vec<String>,
            pub version: Vec<String>,
            pub uptime: Vec<i64>,
            pub os: Vec<String>,
            pub systime: Vec<i64>,
            pub rover_id: Vec<i32>,
            pub rover_name: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing StatusGet200Response".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(
                        <models::DaemonStatus as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "error_message" => intermediate_rep.error_message.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "version" => intermediate_rep.version.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "uptime" => intermediate_rep.uptime.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "os" => intermediate_rep.os.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "systime" => intermediate_rep.systime.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "rover_id" => intermediate_rep.rover_id.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "rover_name" => intermediate_rep.rover_name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing StatusGet200Response".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(StatusGet200Response {
            status: intermediate_rep
                .status
                .into_iter()
                .next()
                .ok_or_else(|| "status missing in StatusGet200Response".to_string())?,
            error_message: intermediate_rep.error_message.into_iter().next(),
            version: intermediate_rep
                .version
                .into_iter()
                .next()
                .ok_or_else(|| "version missing in StatusGet200Response".to_string())?,
            uptime: intermediate_rep
                .uptime
                .into_iter()
                .next()
                .ok_or_else(|| "uptime missing in StatusGet200Response".to_string())?,
            os: intermediate_rep
                .os
                .into_iter()
                .next()
                .ok_or_else(|| "os missing in StatusGet200Response".to_string())?,
            systime: intermediate_rep
                .systime
                .into_iter()
                .next()
                .ok_or_else(|| "systime missing in StatusGet200Response".to_string())?,
            rover_id: intermediate_rep.rover_id.into_iter().next(),
            rover_name: intermediate_rep.rover_name.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<StatusGet200Response> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<StatusGet200Response>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<StatusGet200Response>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for StatusGet200Response - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<StatusGet200Response> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <StatusGet200Response as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into StatusGet200Response - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

/// UnmetServiceError

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UnmetServiceError {
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

impl UnmetServiceError {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> UnmetServiceError {
        UnmetServiceError {
            source: None,
            target: None,
        }
    }
}

/// Converts the UnmetServiceError value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UnmetServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            self.source
                .as_ref()
                .map(|source| ["source".to_string(), source.to_string()].join(",")),
            self.target
                .as_ref()
                .map(|target| ["target".to_string(), target.to_string()].join(",")),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UnmetServiceError value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UnmetServiceError {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub source: Vec<String>,
            pub target: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing UnmetServiceError".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "source" => intermediate_rep.source.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "target" => intermediate_rep.target.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing UnmetServiceError".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UnmetServiceError {
            source: intermediate_rep.source.into_iter().next(),
            target: intermediate_rep.target.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UnmetServiceError> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UnmetServiceError>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<UnmetServiceError>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for UnmetServiceError - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UnmetServiceError> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <UnmetServiceError as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into UnmetServiceError - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

/// UnmetStreamError

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UnmetStreamError {
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    #[serde(rename = "stream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<String>,
}

impl UnmetStreamError {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> UnmetStreamError {
        UnmetStreamError {
            source: None,
            target: None,
            stream: None,
        }
    }
}

/// Converts the UnmetStreamError value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UnmetStreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            self.source
                .as_ref()
                .map(|source| ["source".to_string(), source.to_string()].join(",")),
            self.target
                .as_ref()
                .map(|target| ["target".to_string(), target.to_string()].join(",")),
            self.stream
                .as_ref()
                .map(|stream| ["stream".to_string(), stream.to_string()].join(",")),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UnmetStreamError value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UnmetStreamError {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub source: Vec<String>,
            pub target: Vec<String>,
            pub stream: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing UnmetStreamError".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "source" => intermediate_rep.source.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "target" => intermediate_rep.target.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "stream" => intermediate_rep.stream.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing UnmetStreamError".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UnmetStreamError {
            source: intermediate_rep.source.into_iter().next(),
            target: intermediate_rep.target.into_iter().next(),
            stream: intermediate_rep.stream.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UnmetStreamError> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UnmetStreamError>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<UnmetStreamError>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for UnmetStreamError - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UnmetStreamError> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <UnmetStreamError as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into UnmetStreamError - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdatePost200Response {
    /// The version of the roverd daemon updated to
    #[serde(rename = "version")]
    pub version: String,
}

impl UpdatePost200Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(version: String) -> UpdatePost200Response {
        UpdatePost200Response { version }
    }
}

/// Converts the UpdatePost200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UpdatePost200Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> =
            vec![Some("version".to_string()), Some(self.version.to_string())];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UpdatePost200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UpdatePost200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub version: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing UpdatePost200Response".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "version" => intermediate_rep.version.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing UpdatePost200Response".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UpdatePost200Response {
            version: intermediate_rep
                .version
                .into_iter()
                .next()
                .ok_or_else(|| "version missing in UpdatePost200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UpdatePost200Response> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UpdatePost200Response>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<UpdatePost200Response>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for UpdatePost200Response - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UpdatePost200Response> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <UpdatePost200Response as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into UpdatePost200Response - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}
