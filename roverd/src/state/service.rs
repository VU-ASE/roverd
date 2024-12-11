use std::{fmt::Display, fs, path::Path};

use crate::{
    util::{download_and_install_service, list_dir_contents, update_config},
    Error,
};

use crate::error;

use rovervalidate::{config::Validate, service::ValidatedService};
use tokio::sync::RwLockWriteGuard;

use crate::constants::*;

use openapi::models::*;



pub struct FqVec<'a>(pub Vec<FqService<'a>>);

/// Internal representation of a service, whether as a source or user service.
#[derive(Debug)]
pub struct FqService<'a> {
    pub author: &'a str,
    pub name: &'a str,
    pub version: &'a str,
}

/// Same as FqService but with Strings instead of &str.
#[derive(Debug)]
pub struct FqServiceBuf {
    pub author: String,
    pub name: String,
    pub version: String,
}

impl From<ValidatedService> for FqServiceBuf {
    fn from(service: ValidatedService) -> Self {
        FqServiceBuf {
            name: service.0.name,
            author: service.0.author,
            version: service.0.version,
        }
    }
}

impl Display for FqServiceBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}/{}", self.author, self.name, self.version)?;
        Ok(())
    }
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

impl<'a> From<&'a FqServiceBuf> for FqService<'a> {
    fn from(param: &'a FqServiceBuf) -> Self {
        FqService {
            name: &param.name,
            author: &param.author,
            version: &param.version,
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


