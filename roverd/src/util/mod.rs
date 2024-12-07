use std::{
    fs,
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

// use reqwest::StatusCode;
use axum::http::StatusCode;

use tracing::{info, warn};

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

/// Deletes the contents of a given directory
fn delete_directory_contents(full_path: &Path) -> Result<(), Error> {
    // Iterate over all entries in the directory
    for entry in std::fs::read_dir(full_path)? {
        let entry = entry?;
        let path = entry.path();
        // Recursively delete subdirectories
        if path.is_dir() {
            delete_directory_contents(&path)?;
        }

        // Delete regular files
        if path.is_file() {
            std::fs::remove_file(&path)?;
        }
    }

    Ok(())
}

/// Makes sure the directories for a given service exist. If there is an
/// existing service at a given path it will delete it and prepare it such
/// that the new service can be safely moved in place.
fn prepare_dirs(author: &str, name: &str, version: &str) -> Result<String, Error> {
    // Construct the full path
    let full_path_string = format!("{ROVER_DIR}/{author}/{name}/{version}");
    let full_path = PathBuf::from(full_path_string.clone());

    // Ensure the directory exists
    std::fs::create_dir_all(full_path.clone())?;

    delete_directory_contents(full_path.as_path())?;

    Ok(full_path_string)
}

/// Downloads the vu-ase service from the downloads page and creates a zip file
/// /tmp/name-version.zip.
pub async fn download_service(name: &str, version: &str) -> Result<String, Error> {
    let url = format!("{}/api/{}/v{}", DOWNLOAD_URL, name, version);

    info!("Downloading: {}", url);

    if name.contains(char::is_whitespace) || version.contains(char::is_whitespace) {
        return Err(Error::Download);
    }

    let zip_file = format!("{DOWNLOAD_DESTINATION}/{name}-{version}.zip");

    let response = reqwest::get(url).await?;

    if response.status() != StatusCode::OK {
        let resp: axum::http::StatusCode = response.status();
        match response.status() {
            StatusCode::NOT_FOUND => return Err(Error::ServiceNotFound),
            StatusCode::BAD_REQUEST => return Err(Error::DownloadServiceError),
            _ => return Err(Error::Http(resp)),
        }
    }


    let mut file = std::fs::File::create(zip_file.clone())?;

    let bytes = response.bytes().await?;

    file.write_all(&bytes)?;

    Ok(zip_file)
}

/// Source doesn't exist yet, so download it to /tmp and move it correct place on disk.
/// There shouldn't be any directories or files in the unique path of the service,
/// however if there are, they will get deleted to make space.
pub async fn download_and_install_service<'a>(fq: &FqService<'a>) -> Result<(), Error> {
    let contents_dir = format!(
        "{DOWNLOAD_DESTINATION}/{}-{}-{}",
        fq.author, fq.name, fq.version
    );
    let zip_file = download_service(fq.name, fq.version).await?;

    // Deletes any existing files/dirs that are on the /author/name/version path
    // Makes sure the directories exist.
    let full_path = prepare_dirs(fq.author, fq.name, fq.version)?;

    // Unpack the downloaded service and validate it.
    extract_zip(&zip_file, &contents_dir)?;

    // Copy contents into place
    copy_recursively(contents_dir, full_path)?;

    Ok(())
}

/// Check Check wh
pub fn service_exists<'a>(fq: &FqService<'a>) -> Result<bool, Error> {
    let full_path_string = format!("{ROVER_DIR}/{}/{}/{}", fq.author, fq.name, fq.version);
    match Path::new(full_path_string.as_str()).try_exists() {
        Ok(a) => return Ok(a),
        Err(e) => return Err(Error::Io(e)),
    };
}



#[macro_export]
macro_rules! unwrap_generic {
    ($expr:expr, $error_type:ty) => {{
        match $expr {
            Ok(data) => data,
            Err(e) => {
                warn!("{:#?}", e);
                return Ok(<$error_type>::Status400_AnErrorOccurred(
                    GenericError {
                        message: Some(format!("{:?}", e)),
                        code: Some(1),
                    }
                ));
            }
        }
    }};

    // Optional: Add a variant that allows custom error code
    // ($expr:expr, $error_response_type:path, $error_code:expr) => {{
    //     match $expr {
    //         Ok(data) => data,
    //         Err(e) => {
    //             warn!("{:#?}", e);
    //             return Ok($error_response_type::Status400_AnErrorOccurred(
    //                 GenericError {
    //                     message: Some(format!("{:?}", e)),
    //                     code: Some($error_code),
    //                 }
    //             ));
    //         }
    //     }
    // }};
}
