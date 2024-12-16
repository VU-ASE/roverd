use std::{fmt::Display, fs, path::Path};

use crate::{
    util::{download_and_install_service, list_dir_contents, update_config},
    Error,
};

use crate::error;

use rovervalidate::{config::Validate, service::ValidatedService};
use tokio::sync::RwLockWriteGuard;
use tracing::warn;

use crate::constants::*;

use openapi::models::*;

pub struct FqVec<'a>(pub Vec<Fq<'a>>);

pub struct FqBufVec(pub Vec<FqBuf>);

/// Internal representation of a service, whether as a source or user service.
#[derive(Debug)]
pub struct Fq<'a> {
    pub author: &'a str,
    pub name: &'a str,
    pub version: &'a str,
}

/// Same as FqService but with Strings instead of &str.
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct FqBuf {
    pub author: String,
    pub name: String,
    pub version: String,
}

impl Clone for FqBuf {
    fn clone(&self) -> Self {
        Self {
            author: self.author.clone(),
            name: self.name.clone(),
            version: self.version.clone(),
        }
    }
}

impl From<ValidatedService> for FqBuf {
    fn from(service: ValidatedService) -> Self {
        FqBuf {
            name: service.0.name,
            author: service.0.author,
            version: service.0.version,
        }
    }
}

impl From<&ValidatedService> for FqBuf {
    fn from(service: &ValidatedService) -> Self {
        FqBuf {
            name: service.0.name.clone(),
            author: service.0.author.clone(),
            version: service.0.version.clone(),
        }
    }
}

impl From<&PipelinePostRequestInner> for FqBuf {
    fn from(service: &PipelinePostRequestInner) -> Self {
        FqBuf {
            name: service.name.clone(),
            author: service.author.clone(),
            version: service.version.clone(),
        }
    }
}

impl FqBuf {
    pub fn path(&self) -> String {
        format!(
            "{}/{}/{}/{}/service.yaml",
            ROVER_DIR, self.author, self.name, self.version
        )
    }

    pub fn log_file(&self) -> String {
        format!(
            "{}/{}-{}-{}.log",
            LOG_DIR, self.author, self.name, self.version
        )
    }

    pub fn dir(&self) -> String {
        format!(
            "{}/{}/{}/{}",
            ROVER_DIR, self.author, self.name, self.version
        )
    }
}

impl Display for FqBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}/{}", self.author, self.name, self.version)?;
        Ok(())
    }
}

impl<'a> TryFrom<&'a String> for Fq<'a> {
    type Error = error::Error;
    fn try_from(path_string: &'a String) -> Result<Self, Self::Error> {
        let path = Path::new(path_string.as_str());
        let path_vec: Vec<_> = path.components().collect();

        let num_directory_levels = path_vec.len();
        if num_directory_levels < 3 {
            return Err(Error::EnabledPathInvalid);
        }

        let values = path_vec[(path_vec.len() - 4)..(path_vec.len() - 1)]
            .iter()
            .map(|component| {
                component
                    .as_os_str()
                    .to_str()
                    .ok_or(Error::StringToFqServiceConversion)
            })
            .collect::<Result<Vec<&str>, Error>>()?;

        Ok(Fq {
            author: values.first().ok_or(Error::StringToFqServiceConversion)?,
            name: values.get(1).ok_or(Error::StringToFqServiceConversion)?,
            version: values.get(2).ok_or(Error::StringToFqServiceConversion)?,
        })
    }
}

impl TryFrom<String> for FqBuf {
    type Error = error::Error;
    fn try_from(path_string: String) -> Result<Self, Self::Error> {
        let path = Path::new(path_string.as_str());
        let path_vec: Vec<_> = path.components().collect();

        let num_directory_levels = path_vec.len();
        if num_directory_levels < 3 {
            return Err(Error::EnabledPathInvalid);
        }

        // TODO this is error prone since we extracting the author, name, version from the path
        let values = path_vec[(path_vec.len() - 4)..(path_vec.len() - 1)]
            .iter()
            .map(|component| Ok(component.as_os_str().to_os_string().into_string()?))
            .collect::<Result<Vec<String>, Error>>()?;

        Ok(FqBuf {
            author: values
                .get(0)
                .ok_or(Error::StringToFqServiceConversion)?
                .clone(),
            name: values
                .get(1)
                .ok_or(Error::StringToFqServiceConversion)?
                .clone(),
            version: values
                .get(2)
                .ok_or(Error::StringToFqServiceConversion)?
                .clone(),
        })
    }
}

impl<'a> TryFrom<&'a Vec<String>> for FqVec<'a> {
    type Error = error::Error;
    fn try_from(string_vec: &'a Vec<String>) -> Result<Self, Self::Error> {
        let fq_services: Vec<Fq<'a>> = string_vec
            .iter()
            .map(Fq::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(FqVec(fq_services))
    }
}

impl<'a> From<&'a Vec<PipelinePostRequestInner>> for FqVec<'a> {
    fn from(vec: &'a Vec<PipelinePostRequestInner>) -> Self {
        let fq_services = vec.iter().map(|p| Fq::from(p)).collect::<Vec<_>>();
        FqVec(fq_services)
    }
}

impl From<Vec<PipelinePostRequestInner>> for FqBufVec {
    fn from(vec: Vec<PipelinePostRequestInner>) -> Self {
        let fq_services = vec.iter().map(|p| FqBuf::from(p)).collect::<Vec<_>>();
        FqBufVec(fq_services)
    }
}

impl<'a> Fq<'a> {
    pub fn path(&self) -> String {
        format!(
            "{}/{}/{}/{}/service.yaml",
            ROVER_DIR, self.author, self.name, self.version
        )
    }
}

impl<'a> Fq<'a> {
    pub fn dir(&self) -> String {
        format!(
            "{}/{}/{}/{}",
            ROVER_DIR, self.author, self.name, self.version
        )
    }
}

impl<'a> Display for Fq<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}/{}", self.author, self.name, self.version)?;
        Ok(())
    }
}

impl<'a> From<&'a PipelinePostRequestInner> for Fq<'a> {
    fn from(p: &'a PipelinePostRequestInner) -> Self {
        Fq {
            name: &p.name,
            author: &p.author,
            version: &p.version,
        }
    }
}

impl<'a> From<&'a ServicesAuthorServiceVersionDeletePathParams> for Fq<'a> {
    fn from(param: &'a ServicesAuthorServiceVersionDeletePathParams) -> Self {
        Fq {
            name: &param.service,
            author: &param.author,
            version: &param.version,
        }
    }
}

impl<'a> From<&'a FqBuf> for Fq<'a> {
    fn from(param: &'a FqBuf) -> Self {
        Fq {
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

impl<'a> PartialEq for Fq<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name.to_lowercase() == other.name.to_lowercase()
            && self.author.to_lowercase() == other.author.to_lowercase()
            && self.version.to_lowercase() == other.version.to_lowercase()
    }
}
