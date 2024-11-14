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

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SourcesNameDeletePathParams {
    /// The name of the source
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SourcesNamePostPathParams {
    /// The name of the source
    pub name: String,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<models::PipelineStatus>,

    /// Milliseconds since epoch when the pipeline was manually started
    #[serde(rename = "last_start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_start: Option<i64>,

    /// Milliseconds since epoch when the pipeline was manually stopped
    #[serde(rename = "last_stop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_stop: Option<i64>,

    /// Milliseconds since epoch when the pipeline was automatically restarted (on process faults)
    #[serde(rename = "last_restart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_restart: Option<i64>,
}

impl PipelineGet200ResponsePipeline {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> PipelineGet200ResponsePipeline {
        PipelineGet200ResponsePipeline {
            status: None,
            last_start: None,
            last_stop: None,
            last_restart: None,
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
            self.last_start
                .as_ref()
                .map(|last_start| ["last_start".to_string(), last_start.to_string()].join(",")),
            self.last_stop
                .as_ref()
                .map(|last_stop| ["last_stop".to_string(), last_stop.to_string()].join(",")),
            self.last_restart.as_ref().map(|last_restart| {
                ["last_restart".to_string(), last_restart.to_string()].join(",")
            }),
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
                    "status" => intermediate_rep.status.push(
                        <models::PipelineStatus as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "last_start" => intermediate_rep.last_start.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "last_stop" => intermediate_rep.last_stop.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "last_restart" => intermediate_rep.last_restart.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing PipelineGet200ResponsePipeline"
                                .to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PipelineGet200ResponsePipeline {
            status: intermediate_rep.status.into_iter().next(),
            last_start: intermediate_rep.last_start.into_iter().next(),
            last_stop: intermediate_rep.last_stop.into_iter().next(),
            last_restart: intermediate_rep.last_restart.into_iter().next(),
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

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PipelineGet200ResponseProcessesInner {
    /// The name of the service running as a process
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<models::ProcessStatus>,

    /// The process ID
    #[serde(rename = "pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i32>,

    /// The number of milliseconds the process has been running
    #[serde(rename = "uptime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uptime: Option<i64>,

    /// The amount of memory used by the process in megabytes
    #[serde(rename = "memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i32>,

    /// The percentage of CPU used by the process
    #[serde(rename = "cpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<i32>,

    /// The number of faults that have occurred (causing the pipeline to restart) since last_start
    #[serde(rename = "faults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faults: Option<i32>,
}

impl PipelineGet200ResponseProcessesInner {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> PipelineGet200ResponseProcessesInner {
        PipelineGet200ResponseProcessesInner {
            name: None,
            status: None,
            pid: None,
            uptime: None,
            memory: None,
            cpu: None,
            faults: None,
        }
    }
}

/// Converts the PipelineGet200ResponseProcessesInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for PipelineGet200ResponseProcessesInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            self.name
                .as_ref()
                .map(|name| ["name".to_string(), name.to_string()].join(",")),
            // Skipping status in query parameter serialization
            self.pid
                .as_ref()
                .map(|pid| ["pid".to_string(), pid.to_string()].join(",")),
            self.uptime
                .as_ref()
                .map(|uptime| ["uptime".to_string(), uptime.to_string()].join(",")),
            self.memory
                .as_ref()
                .map(|memory| ["memory".to_string(), memory.to_string()].join(",")),
            self.cpu
                .as_ref()
                .map(|cpu| ["cpu".to_string(), cpu.to_string()].join(",")),
            self.faults
                .as_ref()
                .map(|faults| ["faults".to_string(), faults.to_string()].join(",")),
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
            name: intermediate_rep.name.into_iter().next(),
            status: intermediate_rep.status.into_iter().next(),
            pid: intermediate_rep.pid.into_iter().next(),
            uptime: intermediate_rep.uptime.into_iter().next(),
            memory: intermediate_rep.memory.into_iter().next(),
            cpu: intermediate_rep.cpu.into_iter().next(),
            faults: intermediate_rep.faults.into_iter().next(),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<models::ProcessStatus>,

    /// The process ID
    #[serde(rename = "pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i32>,

    /// The number of milliseconds the process has been running
    #[serde(rename = "uptime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uptime: Option<i64>,

    /// The amount of memory used by the process in megabytes
    #[serde(rename = "memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i32>,

    /// The percentage of CPU used by the process
    #[serde(rename = "cpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<i32>,

    /// The number of faults that have occurred (causing the pipeline to restart) since last_start
    #[serde(rename = "faults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faults: Option<i32>,

    /// The name of the service that this process is running
    #[serde(rename = "service_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,

    /// The version of the service that this process is running
    #[serde(rename = "service_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_version: Option<String>,

    /// The latest <log_lines> log lines of the process
    #[serde(rename = "logs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<Vec<String>>,
}

impl PipelineNameGet200Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> PipelineNameGet200Response {
        PipelineNameGet200Response {
            name: None,
            status: None,
            pid: None,
            uptime: None,
            memory: None,
            cpu: None,
            faults: None,
            service_name: None,
            service_version: None,
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
            self.name
                .as_ref()
                .map(|name| ["name".to_string(), name.to_string()].join(",")),
            // Skipping status in query parameter serialization
            self.pid
                .as_ref()
                .map(|pid| ["pid".to_string(), pid.to_string()].join(",")),
            self.uptime
                .as_ref()
                .map(|uptime| ["uptime".to_string(), uptime.to_string()].join(",")),
            self.memory
                .as_ref()
                .map(|memory| ["memory".to_string(), memory.to_string()].join(",")),
            self.cpu
                .as_ref()
                .map(|cpu| ["cpu".to_string(), cpu.to_string()].join(",")),
            self.faults
                .as_ref()
                .map(|faults| ["faults".to_string(), faults.to_string()].join(",")),
            self.service_name.as_ref().map(|service_name| {
                ["service_name".to_string(), service_name.to_string()].join(",")
            }),
            self.service_version.as_ref().map(|service_version| {
                ["service_version".to_string(), service_version.to_string()].join(",")
            }),
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
            name: intermediate_rep.name.into_iter().next(),
            status: intermediate_rep.status.into_iter().next(),
            pid: intermediate_rep.pid.into_iter().next(),
            uptime: intermediate_rep.uptime.into_iter().next(),
            memory: intermediate_rep.memory.into_iter().next(),
            cpu: intermediate_rep.cpu.into_iter().next(),
            faults: intermediate_rep.faults.into_iter().next(),
            service_name: intermediate_rep.service_name.into_iter().next(),
            service_version: intermediate_rep.service_version.into_iter().next(),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<models::ServiceStatus>,

    /// The version that is enabled for this service (if any)
    #[serde(rename = "enabled_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_version: Option<String>,
}

impl ServicesGet200ResponseInner {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> ServicesGet200ResponseInner {
        ServicesGet200ResponseInner {
            name: None,
            status: None,
            enabled_version: None,
        }
    }
}

/// Converts the ServicesGet200ResponseInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ServicesGet200ResponseInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            self.name
                .as_ref()
                .map(|name| ["name".to_string(), name.to_string()].join(",")),
            // Skipping status in query parameter serialization
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
            name: intermediate_rep.name.into_iter().next(),
            status: intermediate_rep.status.into_iter().next(),
            enabled_version: intermediate_rep.enabled_version.into_iter().next(),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<models::ServiceStatus>,

    #[serde(rename = "versions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<String>>,

    /// The version that is enabled for this service (if any)
    #[serde(rename = "enabled_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_version: Option<String>,
}

impl ServicesNameGet200Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> ServicesNameGet200Response {
        ServicesNameGet200Response {
            name: None,
            status: None,
            versions: None,
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
            self.name
                .as_ref()
                .map(|name| ["name".to_string(), name.to_string()].join(",")),
            // Skipping status in query parameter serialization
            self.versions.as_ref().map(|versions| {
                [
                    "versions".to_string(),
                    versions
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                ]
                .join(",")
            }),
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
            name: intermediate_rep.name.into_iter().next(),
            status: intermediate_rep.status.into_iter().next(),
            versions: intermediate_rep.versions.into_iter().next(),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The version of the service
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<models::ServiceStatus>,

    /// The time this version was last built as milliseconds since epoch
    #[serde(rename = "built_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub built_at: Option<i64>,

    /// The author of the service
    #[serde(rename = "author")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    /// The dependencies/inputs of this service version
    #[serde(rename = "inputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<models::ServicesNameVersionGet200ResponseInputsInner>>,

    /// The output streams of this service version
    #[serde(rename = "outputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<String>>,

    /// The validation errors of this service version (one error per line)
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
}

impl ServicesNameVersionGet200Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> ServicesNameVersionGet200Response {
        ServicesNameVersionGet200Response {
            name: None,
            version: None,
            status: None,
            built_at: None,
            author: None,
            inputs: None,
            outputs: None,
            errors: None,
        }
    }
}

/// Converts the ServicesNameVersionGet200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ServicesNameVersionGet200Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            self.name
                .as_ref()
                .map(|name| ["name".to_string(), name.to_string()].join(",")),
            self.version
                .as_ref()
                .map(|version| ["version".to_string(), version.to_string()].join(",")),
            // Skipping status in query parameter serialization
            self.built_at
                .as_ref()
                .map(|built_at| ["built_at".to_string(), built_at.to_string()].join(",")),
            self.author
                .as_ref()
                .map(|author| ["author".to_string(), author.to_string()].join(",")),
            // Skipping inputs in query parameter serialization
            self.outputs.as_ref().map(|outputs| {
                [
                    "outputs".to_string(),
                    outputs
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                ]
                .join(",")
            }),
            self.errors.as_ref().map(|errors| {
                [
                    "errors".to_string(),
                    errors
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
            name: intermediate_rep.name.into_iter().next(),
            version: intermediate_rep.version.into_iter().next(),
            status: intermediate_rep.status.into_iter().next(),
            built_at: intermediate_rep.built_at.into_iter().next(),
            author: intermediate_rep.author.into_iter().next(),
            inputs: intermediate_rep.inputs.into_iter().next(),
            outputs: intermediate_rep.outputs.into_iter().next(),
            errors: intermediate_rep.errors.into_iter().next(),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The version of the service
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl ServicesPost200Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> ServicesPost200Response {
        ServicesPost200Response {
            name: None,
            version: None,
        }
    }
}

/// Converts the ServicesPost200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ServicesPost200Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            self.name
                .as_ref()
                .map(|name| ["name".to_string(), name.to_string()].join(",")),
            self.version
                .as_ref()
                .map(|version| ["version".to_string(), version.to_string()].join(",")),
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
            name: intermediate_rep.name.into_iter().next(),
            version: intermediate_rep.version.into_iter().next(),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The URL of the source (without scheme)
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// The SHA256 hash of the source download, computed over the ZIP file downloaded
    #[serde(rename = "sha")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha: Option<String>,
}

impl SourcesGet200ResponseInner {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> SourcesGet200ResponseInner {
        SourcesGet200ResponseInner {
            name: None,
            url: None,
            version: None,
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
            self.name
                .as_ref()
                .map(|name| ["name".to_string(), name.to_string()].join(",")),
            self.url
                .as_ref()
                .map(|url| ["url".to_string(), url.to_string()].join(",")),
            self.version
                .as_ref()
                .map(|version| ["version".to_string(), version.to_string()].join(",")),
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
            name: intermediate_rep.name.into_iter().next(),
            url: intermediate_rep.url.into_iter().next(),
            version: intermediate_rep.version.into_iter().next(),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The URL of the source (without scheme)
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// The version of the source
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl SourcesPostRequest {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> SourcesPostRequest {
        SourcesPostRequest {
            name: None,
            url: None,
            version: None,
        }
    }
}

/// Converts the SourcesPostRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for SourcesPostRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            self.name
                .as_ref()
                .map(|name| ["name".to_string(), name.to_string()].join(",")),
            self.url
                .as_ref()
                .map(|url| ["url".to_string(), url.to_string()].join(",")),
            self.version
                .as_ref()
                .map(|version| ["version".to_string(), version.to_string()].join(",")),
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
            name: intermediate_rep.name.into_iter().next(),
            url: intermediate_rep.url.into_iter().next(),
            version: intermediate_rep.version.into_iter().next(),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<models::DaemonStatus>,

    /// Error message of the daemon status
    #[serde(rename = "error_message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,

    /// The version of the roverd daemon
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// The number of milliseconds the roverd daemon process has been running
    #[serde(rename = "uptime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uptime: Option<i64>,

    /// The operating system of the rover
    #[serde(rename = "os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,

    /// The system time of the rover as milliseconds since epoch
    #[serde(rename = "systime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub systime: Option<i64>,

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
    pub fn new() -> StatusGet200Response {
        StatusGet200Response {
            status: None,
            error_message: None,
            version: None,
            uptime: None,
            os: None,
            systime: None,
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
            self.version
                .as_ref()
                .map(|version| ["version".to_string(), version.to_string()].join(",")),
            self.uptime
                .as_ref()
                .map(|uptime| ["uptime".to_string(), uptime.to_string()].join(",")),
            self.os
                .as_ref()
                .map(|os| ["os".to_string(), os.to_string()].join(",")),
            self.systime
                .as_ref()
                .map(|systime| ["systime".to_string(), systime.to_string()].join(",")),
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
            status: intermediate_rep.status.into_iter().next(),
            error_message: intermediate_rep.error_message.into_iter().next(),
            version: intermediate_rep.version.into_iter().next(),
            uptime: intermediate_rep.uptime.into_iter().next(),
            os: intermediate_rep.os.into_iter().next(),
            systime: intermediate_rep.systime.into_iter().next(),
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
