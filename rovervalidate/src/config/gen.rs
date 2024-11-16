/// Example code that deserializes and serializes the model.
/// extern crate serde;
/// #[macro_use]
/// extern crate serde_derive;
/// extern crate serde_json;
///
/// use generated_module::RoverdConfig;
///
/// fn main() {
///     let json = r#"{"answer": 42}"#;
///     let model: RoverdConfig = serde_json::from_str(&json).unwrap();
/// }
use serde_derive::{Deserialize, Serialize};

/// Configuration file for ASE roverlib and rovertui tools, defining services for the
/// pipeline and tracking downloaded services.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration {
    /// List of services downloaded from remote repositories, allowing easy replication of a
    /// pipeline.
    pub downloaded: Vec<Downloaded>,

    /// List of paths to service folders that are enabled. Each folder should contain a
    /// service.yaml file.
    pub enabled: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Downloaded {
    /// The name of the downloaded service.
    pub name: String,

    /// The SHA-1 hash of the downloaded service, ensuring the correct version is used.
    pub sha: Option<String>,

    /// The URL of the remote repository from which the service was downloaded.
    pub source: String,

    /// The version of the downloaded service.
    pub version: String,
}
