use std::fs::{File, OpenOptions};

use std::{
    fs,
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

use axum::http::StatusCode;

use rovervalidate::config::{Configuration, Validate};

use rovervalidate::service::Service;
use tracing::info;

use crate::error::Error;
use crate::service::FqBuf;

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

/// Extracts the contents of the zip file into the directory at destination_dir.
pub fn extract_zip(zip_file: &str, destination_dir: &str) -> Result<(), Error> {
    std::fs::create_dir_all(destination_dir)?;

    let mut file = fs::File::open(zip_file)?;
    let mut bytes: Vec<u8> = Vec::new();
    file.read_to_end(&mut bytes)?;

    let target = PathBuf::from(destination_dir);

    let data_cursor = io::Cursor::new(bytes);
    let mut zip = zip::ZipArchive::new(data_cursor)?;

    zip.extract(target)?;

    Ok(())
}

/// Makes sure the directories for a given service exist. If there is an
/// existing service at a given path it will delete it and prepare it such
/// that the new service can be safely moved in place.
// fn prepare_dirs(author: &str, name: &str, version: &str) -> Result<String, Error> {
fn prepare_dirs(fq: &FqBuf) -> Result<String, Error> {
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
pub async fn download_service(url: &String) -> Result<(), Error> {
    info!("Downloading: {}", url);

    let response = reqwest::get(url).await?;

    if response.status() != StatusCode::OK {
        let resp: axum::http::StatusCode = response.status();
        match response.status() {
            StatusCode::NOT_FOUND => return Err(Error::ServiceNotFound),
            StatusCode::BAD_REQUEST => return Err(Error::ServiceDownloadFailed),
            StatusCode::FORBIDDEN => return Err(Error::Http(StatusCode::FORBIDDEN)),
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
pub async fn download_and_install_service(url: &String) -> Result<FqBuf, Error> {
    download_service(url).await?;
    let fq = extract_fq_from_zip().await?;
    install_service(&fq).await?;
    Ok(fq)
}

/// Attempts to unzip and read out the service in the temporary directory
/// returns the FqBuf on success.
pub async fn extract_fq_from_zip() -> Result<FqBuf, Error> {
    // Clear the destination directory, no matter if it fails
    let _ = std::fs::remove_dir_all(UNZIPPED_DIR);

    // Create directory, this must not fail
    std::fs::create_dir_all(UNZIPPED_DIR)?;

    // Unpack the downloaded service and validate it.
    extract_zip(ZIP_FILE, UNZIPPED_DIR)?;

    // Read contents and
    let service_contents = std::fs::read_to_string(format!("{}/service.yaml", UNZIPPED_DIR))
        .map_err(|_| Error::ServiceYamlNotFoundInDownload)?;
    let service = serde_yaml::from_str::<Service>(&service_contents)?.validate()?;

    let fq = FqBuf::from(service);
    Ok(fq)
}

/// Expects a zipfile to be ready at ZIP_FILE, extract it and install it. Parses the service.yaml
/// and install contents into the correct location on disk.
pub async fn install_service(fq: &FqBuf) -> Result<(), Error> {
    // Deletes any existing files/dirs that are on the /author/name/version path
    // Makes sure the directories exist.
    let full_path = prepare_dirs(fq)?;

    // Copy contents into place
    copy_recursively(UNZIPPED_DIR, full_path)?;

    Ok(())
}

// pub fn service_exists(fq: &Fq<'_>) -> Result<bool, Error> {
//     match Path::new(fq.path().as_str()).try_exists() {
//         Ok(a) => Ok(a),
//         Err(e) => Err(Error::Io(e)),
//     }
// }

pub fn list_dir_contents(added_path: &str) -> Result<Vec<String>, Error> {
    let paths = fs::read_dir(format!("{}/{}", ROVER_DIR, added_path))
        .map_err(|_| Error::ServiceNotFound)?;
    let mut contents: Vec<String> = vec![];

    for path in paths {
        contents.push(path?.file_name().to_os_string().into_string()?)
    }

    Ok(contents)
}

/// Updates the config and creates the file if it doesn't exist
pub fn update_config(config: &Configuration) -> Result<(), Error> {
    let contents = serde_yaml::to_string(&config)?;

    std::fs::create_dir_all(ROVER_CONFIG_DIR).map_err(|_| Error::ConfigFileIO)?;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(ROVER_CONFIG_FILE)
        .map_err(|_| Error::ConfigFileIO)?;

    file.write_all(contents.as_bytes())
        .map_err(|_| Error::ConfigFileIO)?;

    Ok(())
}

pub fn create_log_file(log_path: &PathBuf) -> Result<File, Error> {
    let path = std::path::Path::new(log_path);
    if let Some(parent_dir) = path.parent() {
        if !parent_dir.exists() {
            info!("creating parent dir of logfile: {:?}", &parent_dir);
            std::fs::create_dir_all(parent_dir)?;
        }
    }

    let log_file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(log_path.clone())?;

    Ok(log_file)
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

#[macro_export]
macro_rules! time_now {
    () => {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
    };
}
