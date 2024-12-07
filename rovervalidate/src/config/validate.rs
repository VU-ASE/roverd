use super::gen;

use crate::error::{Error, Result};
use crate::{validate::Validate, validate_field};
use regex::Regex;

// This enforces the type-state pattern, useful for ensuring only accepting valid configurations
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct ValidatedConfiguration(pub gen::Configuration);

/**
 * This module applies validation logic to the auto-generated types for the roverd config. We cannot use the validated crate, because if we regenerate the types, we lose the validation logic.
 * But the validate_field macro gives nicer errors anyway, so it's not a big deal.
 */

impl Validate<ValidatedConfiguration> for gen::Configuration {
    // Validate all properties of the configuration in isolation
    fn validate(&self) -> Result<ValidatedConfiguration> {
        let mut errors = Vec::new();

        // Validate all enabled services, iterate and use the index for every error message
        for (index, enabled) in self.enabled.iter().enumerate() {
            if enabled.is_empty() {
                errors.push(Error::FieldValidationError(crate::error::FieldError {
                    path: vec!["enabled".to_string(), index.to_string()],
                    message: "must not be empty".to_string(),
                }));
            }

            let pattern = Regex::new(r"^\/.*$").unwrap();
            if !pattern.is_match(enabled) {
                errors.push(Error::FieldValidationError(crate::error::FieldError {
                    path: vec!["enabled".to_string(), index.to_string()],
                    message: "must be a valid path that starts with a slash".to_string(),
                }));
            }
        }

        // Validate all downloaded services, iterate and use the index for every error message
        for (index, downloaded) in self.downloaded.iter().enumerate() {
            if let Err(mut downloaded_errors) = downloaded.validate() {
                for error in downloaded_errors.iter_mut() {
                    match error {
                        Error::FieldValidationError(field_error) => {
                            field_error.path.insert(0, "downloaded".to_string());
                            field_error.path.insert(1, index.to_string());
                        }
                        Error::ParseError(parse_error) => {
                            parse_error.path.insert(0, "downloaded".to_string());
                            parse_error.path.insert(1, index.to_string());
                        }
                        _ => (),
                    }
                }
                errors.append(&mut downloaded_errors);
            }
        }

        // Check that there are no duplicate services in the downloaded services
        let mut service_names = Vec::new();
        let mut service_sources = Vec::new();
        for (index, downloaded) in self.downloaded.iter().enumerate() {
            let path = vec!["downloaded".to_string(), index.to_string()];

            if service_names.contains(&downloaded.name) {
                let msg = format!(
                    "names of downloaded services must be unique, but {} exists more than once",
                    downloaded.name
                );

                errors.push(Error::FieldValidationError(crate::error::FieldError {
                    path: path.clone(),
                    message: msg,
                }));
            } else {
                service_names.push(downloaded.name.clone());
            }

            if service_sources.contains(&downloaded.source) {
                let msg = format!(
                    "sources of downloaded services must be unique, but {} exists more than once",
                    downloaded.source
                );

                errors.push(Error::FieldValidationError(crate::error::FieldError {
                    path: path.clone(),
                    message: msg,
                }));
            } else {
                service_sources.push(downloaded.source.clone());
            }
        }

        // Check that there are no duplicate services in the enabled services
        let mut enabled_services = Vec::new();
        for (index, enabled) in self.enabled.iter().enumerate() {
            if enabled_services.contains(&enabled) {
                let msg = format!(
                    "enabled services must be unique, but {} exists more than once",
                    enabled
                );

                errors.push(Error::FieldValidationError(crate::error::FieldError {
                    path: vec!["enabled".to_string(), index.to_string()],
                    message: msg,
                }));
            } else {
                enabled_services.push(enabled);
            }
        }

        if errors.is_empty() {
            Ok(ValidatedConfiguration(self.clone()))
        } else {
            Err(errors)
        }
    }
}

impl Validate<bool> for gen::Downloaded {
    fn validate(&self) -> Result<bool> {
        let mut errors = Vec::new();

        validate_field!(self.name, &mut errors, |value| {
            if value.is_empty() {
                return Some("must not be empty");
            }

            let pattern = Regex::new(r"^[a-z]+(-[a-z]+)*$").unwrap();
            if !pattern.is_match(value) {
                return Some("can only consist of lowercase letters and hyphens");
            }

            None
        });

        validate_field!(self.source, &mut errors, |value| {
            if value.is_empty() {
                return Some("must not be empty");
            }

            let pattern =
                Regex::new(r"^([a-zA-Z0-9.-]+\.[a-zA-Z]{2,})(/[a-zA-Z0-9._~%!$&'()*+,;=:@-]*)*$")
                    .unwrap();
            if !pattern.is_match(value) {
                return Some("must be a valid URL, without a scheme (no http:// or https://)");
            }

            None
        });

        validate_field!(self.version, &mut errors, |value| {
            if value.is_empty() {
                return Some("must not be empty");
            }

            let pattern = Regex::new(r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?$").unwrap();
            if !pattern.is_match(value) {
                return Some("must be a valid semantic version");
            }

            None
        });

        // Validate SHA if present
        validate_field!(self.sha, &mut errors, |value| {
            if let Some(sha) = value {
                let pattern = Regex::new(r"[a-fA-F0-9]{64}$").unwrap();
                if !pattern.is_match(sha) {
                    return Some("must be a valid SHA256 hash or must be omitted");
                }
            }

            None
        });

        if errors.is_empty() {
            Ok(true)
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILES_LOCATION: &str = "./src/testfiles/roverd-yaml";

    #[test]
    fn test_valid_files() {
        let valid_path = format!("{}/valid", TEST_FILES_LOCATION);

        // Get all files in this directory
        let files = std::fs::read_dir(valid_path).unwrap();

        // Iterate over all files and validate them
        for file in files {
            let file = file.unwrap();
            let file_path = file.path();
            let file_name = file.file_name().into_string().unwrap();

            // Skip directories
            if file_path.is_dir() {
                continue;
            }

            println!("Validating file: {}", file_name);

            let file_content = std::fs::read_to_string(file_path).unwrap();
            let service: gen::Configuration = serde_yaml::from_str(&file_content).unwrap();

            // Print errors
            if let Err(errors) = service.validate() {
                for error in errors {
                    print!("{}\n", error);
                }
            }

            assert!(
                service.validate().is_ok(),
                "Validation failed for file: {}",
                file_name
            );
        }
    }

    #[test]
    fn test_invalid_files() {
        let invalid_path = format!("{}/invalid", TEST_FILES_LOCATION);

        // Get all files in this directory
        let files = std::fs::read_dir(invalid_path).unwrap();

        // Iterate over all files and validate them
        for file in files {
            let file = file.unwrap();
            let file_path = file.path();
            let file_name = file.file_name().into_string().unwrap();

            // Skip directories
            if file_path.is_dir() {
                continue;
            }

            println!("Validating file: {}", file_name);

            let file_content = std::fs::read_to_string(file_path).unwrap();
            let service: gen::Configuration = serde_yaml::from_str(&file_content).unwrap();

            assert!(
                service.validate().is_err(),
                "Validation should have failed for file: {}",
                file_name
            );
        }
    }
}
