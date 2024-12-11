use std::{fmt::Display, fs, path::Path};

use crate::{
    util::{list_dir_contents, update_config},
    Error,
};

use crate::error;

use rovervalidate::{config::Validate, service::ValidatedService};
use tokio::sync::RwLockWriteGuard;

use crate::constants::*;

use openapi::models::*;

use super::State;

#[derive(Debug, Clone)]
pub struct Services;

pub struct FqVec<'a>(pub Vec<FqService<'a>>);

/// Internal representation of a service, whether as a source or user service.
#[derive(Debug)]
pub struct FqService<'a> {
    pub author: &'a str,
    pub name: &'a str,
    pub version: &'a str,
}

impl<'a> TryFrom<&'a String> for FqService<'a> {
    type Error = error::Error;
    fn try_from(path_string: &'a String) -> Result<Self, Self::Error> {
        let path = Path::new(path_string.as_str());
        let path_vec: Vec<_> = path.components().collect();

        let num_directory_levels = path_vec.len();
        if num_directory_levels < 3 {
            return Err(Error::EnabledPathInvalid);
        }

        let values = path_vec[path_vec.len() - 3..path_vec.len()]
            .iter()
            .map(|component| {
                component
                    .as_os_str()
                    .to_str()
                    .ok_or(Error::StringToFqServiceConversion)
            })
            .collect::<Result<Vec<&str>, Error>>()?;

        Ok(FqService {
            author: values.first().ok_or(Error::StringToFqServiceConversion)?,
            name: values.get(1).ok_or(Error::StringToFqServiceConversion)?,
            version: values.get(2).ok_or(Error::StringToFqServiceConversion)?,
        })
    }
}

impl<'a> TryFrom<&'a Vec<String>> for FqVec<'a> {
    type Error = error::Error;
    fn try_from(string_vec: &'a Vec<String>) -> Result<Self, Self::Error> {
        let fq_services: Vec<FqService<'a>> = string_vec
            .iter()
            .map(FqService::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(FqVec(fq_services))
    }
}

impl<'a> FqService<'a> {
    pub fn path(&self) -> String {
        format!(
            "{}/{}/{}/{}",
            ROVER_DIR, self.author, self.name, self.version
        )
    }
}

impl<'a> Display for FqService<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}/{}", self.author, self.name, self.version)?;
        Ok(())
    }
}

impl<'a> From<&'a ServicesAuthorServiceVersionDeletePathParams> for FqService<'a> {
    fn from(param: &'a ServicesAuthorServiceVersionDeletePathParams) -> Self {
        FqService {
            name: &param.service,
            author: &param.author,
            version: &param.version,
        }
    }
}

impl<'a> From<&'a ValidatedService> for FqService<'a> {
    fn from(s: &'a ValidatedService) -> Self {
        FqService {
            name: &s.0.name,
            author: &s.0.author,
            version: &s.0.version,
        }
    }
}

// TODO:
// impl<'a> From<&'a FetchPostRequest> for FqService<'a> {
//     fn from(value: &'a FetchPostRequest) -> Self {
//         FqService {
//             name: &value.name,
//             author: AUTHOR,
//             version: &value.version,
//             url: Some(&value.url),
//         }
//     }
// }

impl<'a> PartialEq for FqService<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name.to_lowercase() == other.name.to_lowercase()
            && self.author.to_lowercase() == other.author.to_lowercase()
            && self.version.to_lowercase() == other.version.to_lowercase()
    }
}

impl Services {
    pub async fn get_authors(&self) -> Result<Vec<String>, Error> {
        list_dir_contents("")
    }

    pub async fn get_services(
        &self,
        path_params: ServicesAuthorGetPathParams,
    ) -> Result<Vec<String>, Error> {
        list_dir_contents(&path_params.author.to_string())
    }

    pub async fn get_versions(
        &self,
        path_params: ServicesAuthorServiceGetPathParams,
    ) -> Result<Vec<String>, Error> {
        list_dir_contents(format!("{}/{}", path_params.author, path_params.service).as_str())
    }

    pub async fn get_service(
        &self,
        path_params: ServicesAuthorServiceVersionGetPathParams,
    ) -> Result<ValidatedService, Error> {
        // Load config from file on disk
        let service_file_path = format!(
            "{}/{}/{}/{}/service.yaml",
            ROVER_DIR, path_params.author, path_params.service, path_params.version
        );
        let contents = fs::read_to_string(service_file_path)?;
        let service =
            serde_yaml::from_str::<rovervalidate::service::Service>(&contents)?.validate()?;

        Ok(service)
    }

    pub async fn delete(
        &self,
        state: &RwLockWriteGuard<'_, State>,
        path_params: &ServicesAuthorServiceVersionDeletePathParams,
    ) -> Result<bool, Error> {
        let delete_fq = FqService::from(path_params);

        // Get the current configuration from disk
        let mut config = state.config.get().await?.0;

        let mut return_bool = false;
        // Return whether or not the service was enabled and if it was,
        // reset the pipeline
        let enabled_fq_vec = FqVec::try_from(&config.enabled)?.0;
        if enabled_fq_vec
            .iter()
            .any(|enabled_fq| enabled_fq == &delete_fq)
        {
            config.enabled.clear();
            update_config(&config)?;
            return_bool = true;
        }

        // Remove the service to delete from the filesystem
        std::fs::remove_dir_all(delete_fq.path())?;

        Ok(return_bool)
    }

    pub async fn build_service(
        &self,
        _params: ServicesAuthorServiceVersionPostPathParams,
    ) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }
}
