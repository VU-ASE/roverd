use std::fs::{remove_dir, OpenOptions};

use std::{
    fs,
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

// use reqwest::StatusCode;
use axum::http::StatusCode;

use rovervalidate::config::{Configuration, Validate};
use rovervalidate::service::Service;

use tracing::{error, info};

use crate::error::Error;

use super::state::services::FqService;

use crate::constants::*;

/// Copy files from source to destination recursively.
pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

/// Extracts the contents of the zip file into the directory at
/// destination_dir.
pub fn extract_zip(zip_file: &str, destination_dir: &str) -> Result<(), Error> {
    std::fs::create_dir_all(destination_dir)?;

    let mut file = fs::File::open(zip_file)?;
    let mut bytes: Vec<u8> = Vec::new();
    file.read_to_end(&mut bytes)?;

    let target = PathBuf::from(destination_dir);

    let a = io::Cursor::new(bytes);
    let mut z = zip::ZipArchive::new(a)?;

    z.extract(target)?;

    Ok(())
}

/// Makes sure the directories for a given service exist. If there is an
/// existing service at a given path it will delete it and prepare it such
/// that the new service can be safely moved in place.
// fn prepare_dirs(author: &str, name: &str, version: &str) -> Result<String, Error> {
fn prepare_dirs(fq: &FqService) -> Result<String, Error> {
    // Construct the full path
    let full_path_string = format!("{}/{}/{}/{}", ROVER_DIR, fq.author, fq.name, fq.version);
    let full_path = PathBuf::from(full_path_string.clone());

    // Ensure the directory exists
    std::fs::create_dir_all(full_path.clone())?;

    std::fs::remove_dir_all(full_path.as_path())?;

    Ok(full_path_string)
}

/// Downloads the vu-ase service from the downloads page and creates a zip file
/// /tmp/name-version.zip.
pub async fn download_service(url: String) -> Result<(), Error> {
    info!("Downloading: {}", url);

    let response = reqwest::get(url).await?;

    if response.status() != StatusCode::OK {
        let resp: axum::http::StatusCode = response.status();
        match response.status() {
            StatusCode::NOT_FOUND => return Err(Error::ServiceNotFound),
            StatusCode::BAD_REQUEST => return Err(Error::ServiceDownloadFailed),
            StatusCode::FORBIDDEN => return Err(Error::ServiceNotFound),
            _ => return Err(Error::Http(resp)),
        }
    }

    let mut file = std::fs::File::create(ZIP_FILE)?;

    let bytes = response.bytes().await?;

    file.write_all(&bytes)?;

    Ok(())
}

/// Downloads a service to /tmp and moves it into the correct place on disk.
/// There shouldn't be any directories or files in the unique path of the service,
/// however if there are, they will get deleted to make space.
pub async fn download_and_install_service(url: String) -> Result<(), Error> {
    download_service(url).await?;
    install_service().await?;
    Ok(())
}

/// Given the path of a zipfile, extract it and install it, parse the service.yaml
/// and install it into the correct location on disk.
pub async fn install_service() -> Result<(), Error> {
    // Clear the destination directory
    std::fs::remove_dir_all(UNZIPPED_DIR)?;

    // Create directory
    std::fs::create_dir_all(UNZIPPED_DIR)?;

    // Unpack the downloaded service and validate it.
    extract_zip(ZIP_FILE, UNZIPPED_DIR)?;

    let service_contents = std::fs::read_to_string(format!("{}/service.yaml", UNZIPPED_DIR))
        .map_err(|_| Error::ServiceYamlNotInZip)?;

    let service =
        serde_yaml::from_str::<rovervalidate::service::Service>(&service_contents)?.validate()?;

    let fq = FqService::from(&service);

    // Deletes any existing files/dirs that are on the /author/name/version path
    // Makes sure the directories exist.
    let full_path = prepare_dirs(&fq)?;

    let contents_dir = PathBuf::from(format!(
        "{DOWNLOAD_DESTINATION}/{}-{}-{}",
        fq.author, fq.name, fq.version
    ));

    // Copy contents into place
    copy_recursively(contents_dir, full_path)?;

    Ok(())
}

pub fn service_exists(fq: &FqService<'_>) -> Result<bool, Error> {
    match Path::new(fq.path().as_str()).try_exists() {
        Ok(a) => Ok(a),
        Err(e) => Err(Error::Io(e)),
    }
}

pub fn delete_service_from_disk(fq: &FqService<'_>) -> Result<(), Error> {
    if service_exists(fq)? {
        std::fs::remove_dir_all(fq.path())?;
    }
    Ok(())
}

pub fn list_dir_contents(added_path: &str) -> Result<Vec<String>, Error> {
    let paths = fs::read_dir(format!("{}/{}", ROVER_DIR, added_path))?;
    let mut contents: Vec<String> = vec![];

    for path in paths {
        contents.push(path?.file_name().to_os_string().into_string()?)
    }

    Ok(contents)
}

/// Updates the config and creates the file if it doesn't exist
pub fn update_config(config: &Configuration) -> Result<(), Error> {
    let contents = serde_yaml::to_string(&config)?;

    std::fs::create_dir_all(ROVER_CONFIG_DIR).inspect_err(|f| {
        error!("Could not create {f}");
    })?;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(ROVER_CONFIG_FILE)
        .map_err(|_| Error::CouldNotCreateConfigFile)?;

    file.write_all(contents.as_bytes())
        .map_err(|_| Error::CouldNotWriteToConfigFile)?;

    Ok(())
}

#[macro_export]
macro_rules! warn_generic {
    ($expr:expr, $error_type:ty) => {{
        match $expr {
            Ok(data) => data,
            Err(e) => {
                warn!("{:#?}", e);
                return Ok(<$error_type>::Status400_AnErrorOccurred(GenericError {
                    message: Some(format!("{:?}", e)),
                    code: Some(1),
                }));
            }
        }
    }};
}

#[macro_export]
macro_rules! error_generic {
    ($expr:expr, $error_type:ty) => {{
        match $expr {
            Ok(data) => data,
            Err(e) => {
                error!("{:#?}", e);
                return Ok(<$error_type>::Status400_AnErrorOccurred(GenericError {
                    message: Some(format!("{:?}", e)),
                    code: Some(1),
                }));
            }
        }
    }};
}
