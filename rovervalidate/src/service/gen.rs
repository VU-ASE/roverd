// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::Service;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: Service = serde_json::from_str(&json).unwrap();
// }

use serde_derive::{Deserialize, Serialize};

/// Configuration file for a service in the ASE Rover platform, defining service identity,
/// commands, data streams, and runtime options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    /// The author of the service.
    pub author: String,

    /// Commands to build and run the service. Executed from the service folder.
    pub commands: Commands,

    /// List of configuration options that can be accessed during runtime.
    pub configuration: Vec<Configuration>,

    /// List of input streams this service consumes from other services.
    pub inputs: Vec<Input>,

    /// The name of the service.
    pub name: String,

    /// Names of the streams that this service produces.
    pub outputs: Vec<String>,

    /// URL of the service's source repository.
    pub source: String,

    /// The version of the service.
    pub version: String,
}

/// Commands to build and run the service. Executed from the service folder.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commands {
    /// The command to build the service. Optional if no build step is involved.
    pub build: Option<String>,

    /// The command to run the service.
    pub run: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    /// The name of the configuration option.
    pub name: String,

    /// Indicates if the configuration option can be changed during runtime.
    pub tunable: Option<bool>,

    /// Specifies the type of the configuration value if it needs to override auto-detection
    /// (options: string, float).
    #[serde(rename = "type")]
    pub configuration_type: Option<Type>,

    /// The value of the configuration option, which can be a string or float.
    pub value: Value,
}

/// Specifies the type of the configuration value if it needs to override auto-detection
/// (options: string, float).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Number,
    String,
}

/// The value of the configuration option, which can be a string or float.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Number(f64),
    String(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    /// The name of the service providing the input streams.
    pub service: String,

    /// List of streams from the specified service that this service consumes.
    pub streams: Vec<String>,
}
