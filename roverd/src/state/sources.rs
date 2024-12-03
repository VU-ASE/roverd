use std::fs::write;

use crate::error::Error;

use openapi::models::SourcesPostRequest;
use rovervalidate::config::{Configuration, Downloaded, Validate, ValidatedConfiguration};

use crate::util::download_and_install_service;

use tracing::error;

use crate::constants::*;

/// Data structure that holds the run-time mutable configuration of the rover.
/// Reflective of a valid /etc/roverd/rover.yaml configurtaion file.
#[derive(Debug, Clone)]
pub struct Sources;

impl Sources {
    /// Retrieves rover.yaml file from disk, performs validation and returns object.
    pub fn get(&self) -> Result<rovervalidate::config::ValidatedConfiguration, Error> {
        let file_content =
            std::fs::read_to_string(ROVER_CONFIG_FILE).map_err(|_| Error::ConfigFileNotFound)?;

        let config: ValidatedConfiguration =
            serde_yaml::from_str::<Configuration>(&file_content)?.validate()?;

        Ok(config)
    }

    pub async fn add(&self, source: SourcesPostRequest) -> Result<(), Error> {
        // First, check if the source to add already exists.
        let mut config = self.get()?.0;

        let incoming_source = Downloaded {
            sha: None,
            name: source.name.to_lowercase(),
            source: source.url.to_lowercase(),
            version: source.version.to_lowercase(),
        };

        if incoming_source.source.contains("http") {
            return Err(Error::Source(
                "source url should not contain schema, remove 'http...'".to_string(),
            ));
        }

        for existing_source in &config.downloaded {
            if existing_source.name.to_lowercase() == incoming_source.name
                && existing_source.source.to_lowercase() == incoming_source.source
                && existing_source.version.to_lowercase() == incoming_source.version
            {
                let error_msg = "already exists".to_string();
                return Err(Error::Source(error_msg));
            }
        }

        // Update the config file with the newly added source
        config.downloaded.push(incoming_source);
        let contents = serde_yaml::to_string(&config)?;
        write(ROVER_CONFIG_FILE, contents)?;

        // Based on the updated config file, install the sources
        self.install_missing_sources().await?;

        Ok(())
    }

    /// Idempotently installs any missing sources based on roverd config file.
    async fn install_missing_sources(&self) -> Result<(), Error> {

        let missing_sources: Vec<Downloaded> = vec![];

        // todo: install missing downloads
        // todo: error on missing services


        // // Extract the repository name from the url.
        // let mut url_slice = incoming_source.source.as_str();
        // let slash_index = url_slice.rfind('/').ok_or(Error::Url)?;
        // let url_len = url_slice.len();

        // if slash_index == url_len - 1 {
        //     url_slice = &url_slice[..url_len - 1]
        // }

        // let slash_index = url_slice.rfind('/').ok_or(Error::Url)?;
        // let repo_name = &url_slice[(slash_index + 1)..];

        // download_and_install_service(repo_name, &incoming_source.version).await?;

        Ok(())
    }

    pub async fn delete(&self, _source: SourcesPostRequest) -> Result<(), Error> {
        error!("TODO: unimplemented");
        Ok(())
    }
}
