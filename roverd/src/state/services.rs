use std::fmt::Display;

use crate::Error;

use rovervalidate::{config::Downloaded, service::ValidatedService};

use crate::constants::*;

use openapi::models::*;

#[derive(Debug, Clone)]
pub struct Services;

#[derive(Debug)]
pub struct FqService<'a> {
    pub author: &'a str,
    pub name: &'a str,
    pub version: &'a str,
    pub path: String,
    pub source_url: String,
}

impl<'a> Display for FqService<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}/{}", self.author, self.name, self.version)?;
        Ok(())
    }
}

impl<'a> From<&'a SourcesPostRequest> for FqService<'a> {
    fn from(value: &'a SourcesPostRequest) -> Self {
        FqService {
            name: &value.name,
            author: AUTHOR,
            version: &value.version,
            path: format!(
                "{}/{}/{}/{}",
                ROVER_DIR, AUTHOR, &value.name, &value.version
            ),
            source_url: value.url.to_string(),
        }
    }
}

impl<'a> From<&'a Downloaded> for FqService<'a> {
    fn from(value: &'a Downloaded) -> Self {
        FqService {
            name: &value.name,
            author: AUTHOR,
            version: &value.version,
            path: format!(
                "{}/{}/{}/{}",
                ROVER_DIR, AUTHOR, &value.name, &value.version
            ),
            source_url: value.source.to_string(),
        }
    }
}

impl<'a> PartialEq for FqService<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name.to_lowercase() == other.name.to_lowercase()
            && self.author.to_lowercase() == other.author.to_lowercase()
            && self.version.to_lowercase() == other.version.to_lowercase()
            && self.source_url.to_lowercase() == other.source_url.to_lowercase()
    }
}

impl Services {
    pub async fn get_authors(&self) -> Result<Vec<String>, Error> {
        Ok(vec!["asdf".to_string()])
    }

    pub async fn get_service(
        &self,
        _fq_service: ServicesAuthorServiceVersionGetPathParams,
    ) -> Result<ValidatedService, Error> {
        Err(Error::ConfigFileNotFound)
    }

    pub async fn get_version(
        &self,
        _path_params: ServicesAuthorServiceGetPathParams,
    ) -> Result<Vec<String>, Error> {
        Ok(vec!["asdf".to_string()])
    }
}
