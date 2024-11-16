use std::{io::Write, path::Path, path::PathBuf};



use tracing::info;

use crate::error::Error;

const ROVER_DIR: &str = "/home/debix/.rover";
const AUTHOR: &str = "vu-ase";

const DOWNLOAD_URL: &str = "https://downloads.ase.vu.nl";
const DOWNLOAD_DESTINATION: &str = "/tmp";




/// Extracts the contents of the zip file into the directory at 
/// destination_dir.
fn extract_zip(zip_file: &str, destination_dir: &str) -> Result<(), Error> {

    info!("going to extract {} into {}", zip_file, destination_dir);
    // Ensure the output directory exists
    // fs::create_dir_all(output_dir)?;

    // // Open the zip file
    // let input_path = PathBuf::from(input_path);
    // let output_dir = PathBuf::from(output_dir);
    
    // let archive = zip::ZipArchive::new(input_path)?;
    
    // // Extract each file from the archive
    // for entry in archive.entries() {
    //     let mut entry = entry?;
        
    //     // Skip directories
    //     if entry.file_name().is_none() || entry.name().ends_with('/').unwrap_or(false) {
    //         continue;
    //     }
        
    //     // Create the directory structure if it doesn't exist
    //     let out_path = output_dir.join(entry.name());
    //     let parent = out_path.parent().unwrap_or(&output_dir);
    //     fs::create_dir_all(parent)?;
        
    //     // Extract the file content
    //     let mut dest = fs::File::create(out_path)?;
    //     let mut cursor = Cursor::new(Vec::new());
    //     entry.read_to_end(&mut cursor)?;
    //     dest.write_all(&cursor.get_ref())?;
    // }
    
    Ok(())
}



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
    // Define the base directory
    let base_dir = ROVER_DIR.to_string();

    // Construct the full path
    let full_path_string = format!("{ROVER_DIR}/{author}/{name}/{version}");
    let full_path = PathBuf::from(full_path_string.clone());

    // Ensure the directory exists
    std::fs::create_dir_all(full_path.clone())?;

    delete_directory_contents(full_path.as_path())?;

    Ok(full_path_string)
}

pub async fn download_service(name: &str, version: &str) -> Result<String, Error> {
    let url = format!("{}/api/{}/{}", DOWNLOAD_URL, name, version);

    if name.contains(char::is_whitespace) || version.contains(char::is_whitespace) {
        return Err(Error::Download);
    }

    let zip_file = format!("{DOWNLOAD_DESTINATION}/{name}-{version}.zip");

    let response = reqwest::get(url).await?;
    let mut file = std::fs::File::create(zip_file.clone())?;

    info!("downloading...");
    let bytes = response.bytes().await?;
    
    info!("writing...");
    file.write_all(&bytes)?;

    let full_path = prepare_dirs(&AUTHOR, &name, &version)?;

    extract_zip(&zip_file, &full_path.as_str())?;

    Ok(full_path)
}
