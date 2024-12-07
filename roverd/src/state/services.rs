use crate::Error;

use rovervalidate::service::ValidatedService;

use openapi::models::*;

#[derive(Debug, Clone)]
pub struct Services;

pub struct FqService<'a> {
    pub author: &'a str,
    pub name: &'a str,
    pub version: &'a str,
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
