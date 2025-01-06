use crate::error::{Error, Result};
use crate::{config::Validate, service};

/**
 * The Pipeline defines all services integrated "as a whole". A pipeline is verified once and can then be started as often as needed.
 * As soon as one service in the pipeline is changed (i.e. crashes), the pipeline is considered invalid and should as a whole be restarted (but not necessarily re-verified).
 * It is up to the user of the pipeline interface how to start the pipeline and individual services. All checks are static.
 */

#[derive(Debug, Clone)]
pub struct Pipeline {
    services: Vec<service::ValidatedService>,
}

// Pipelines are immutable, you initialize them once and then they are used as a whole.
// To enforce using only valid pipelines, there is no way to view pipelines directly when not validated yet.
impl Pipeline {
    pub fn new(services: Vec<service::ValidatedService>) -> Self {
        Self { services }
    }
}

impl Validate<RunnablePipeline> for Pipeline {
    fn validate(&self) -> Result<RunnablePipeline> {
        let mut errors = Vec::new();

        // Are all service names unique?
        let mut service_names = Vec::new();
        for service in self.services.iter() {
            if service_names.contains(&service.0.name) {
                errors.push(Error::PipelineValidationError(
                    crate::error::PipelineValidationError::DuplicateServiceError(
                        service.0.name.clone(),
                    ),
                ));
            } else {
                service_names.push(service.0.name.clone());
            }
        }

        // Are all used inputs produced as output by another service?
        for service in self.services.iter() {
            for input in service.0.inputs.iter() {
                for stream in input.streams.iter() {
                    if !self.services.iter().any(|s| {
                        s.0.name == input.service
                            && s.0.name != service.0.name
                            && s.0.outputs.iter().any(|o| o == stream)
                    }) {
                        errors.push(Error::PipelineValidationError(
                            crate::error::PipelineValidationError::UnmetDependencyError(
                                crate::error::UnmetDependencyError::UnmetStream(
                                    crate::error::UnmetStreamError {
                                        source: service.0.name.clone(),
                                        target: input.service.clone(),
                                        stream: stream.clone(),
                                    },
                                ),
                            ),
                        ));
                    }
                }
            }
        }

        if errors.is_empty() {
            Ok(RunnablePipeline(self.clone()))
        } else {
            Err(errors)
        }
    }
}

// This enforces the type-state pattern, useful for ensuring only accepting valid configurations
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct RunnablePipeline(pub Pipeline);

// Within a validated pipeline, you are allowed to view the services, which you can use to start the pipeline.
impl RunnablePipeline {
    pub fn services(&self) -> &Vec<service::ValidatedService> {
        &self.0.services
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::Service;

    #[test]
    fn test_valid_basic_pipeline() {
        // Three services, with a simple A -> B -> C dependency chain
        let a = Service {
            name: "a".to_string(),
            author: "ase-test".to_string(),
            version: "0.1.0".to_string(),
            source: "github.com/ase-test/a".to_string(),
            commands: service::Commands {
                build: None,
                run: "echo 'a'".to_string(),
            },
            inputs: vec![],
            outputs: vec!["a".to_string()],
            configuration: vec![],
        };
        let b = Service {
            name: "b".to_string(),
            author: "ase-test".to_string(),
            version: "0.1.0".to_string(),
            source: "github.com/ase-test/b".to_string(),
            commands: service::Commands {
                build: None,
                run: "echo 'b'".to_string(),
            },
            inputs: vec![service::Input {
                service: "a".to_string(),
                streams: vec!["a".to_string()],
            }],
            outputs: vec!["b".to_string()],
            configuration: vec![],
        };
        let c = Service {
            name: "c".to_string(),
            author: "ase-test".to_string(),
            version: "0.1.0".to_string(),
            source: "github.com/ase-test/c".to_string(),
            commands: service::Commands {
                build: None,
                run: "echo 'c'".to_string(),
            },
            inputs: vec![service::Input {
                service: "b".to_string(),
                streams: vec!["b".to_string()],
            }],
            outputs: vec!["c".to_string()],
            configuration: vec![],
        };

        // Validate all services
        let a = a.validate().unwrap();
        let b = b.validate().unwrap();
        let c = c.validate().unwrap();

        // Create a pipeline
        let pipeline = Pipeline::new(vec![a, b, c]);
        let validated_pipeline = pipeline.validate();

        assert!(validated_pipeline.is_ok());
        // Assert that the pipeline contains the correct services
        let validated_pipeline = validated_pipeline.unwrap();
        assert_eq!(validated_pipeline.services().len(), 3);
        assert_eq!(validated_pipeline.services()[0].0.name, "a");
        assert_eq!(validated_pipeline.services()[1].0.name, "b");
        assert_eq!(validated_pipeline.services()[2].0.name, "c");
    }

    #[test]
    fn test_invalid_pipeline_missing_stream() {
        // Three services, with a simple A -> B -> C dependency chain
        let a = Service {
            name: "a".to_string(),
            author: "ase-test".to_string(),
            version: "0.1.0".to_string(),
            source: "github.com/ase-test/a".to_string(),
            commands: service::Commands {
                build: None,
                run: "echo 'a'".to_string(),
            },
            inputs: vec![],
            outputs: vec!["a".to_string()],
            configuration: vec![],
        };
        let b = Service {
            name: "b".to_string(),
            author: "ase-test".to_string(),
            version: "0.1.0".to_string(),
            source: "github.com/ase-test/b".to_string(),
            commands: service::Commands {
                build: None,
                run: "echo 'b'".to_string(),
            },
            inputs: vec![service::Input {
                service: "a".to_string(),
                streams: vec!["a".to_string()],
            }],
            outputs: vec!["b".to_string()],
            configuration: vec![],
        };
        let c = Service {
            name: "c".to_string(),
            author: "ase-test".to_string(),
            version: "0.1.0".to_string(),
            source: "github.com/ase-test/c".to_string(),
            commands: service::Commands {
                build: None,
                run: "echo 'c'".to_string(),
            },
            inputs: vec![service::Input {
                service: "b".to_string(),
                streams: vec!["c".to_string()], // depends on a stream that does not exist from service b
            }],
            outputs: vec!["c".to_string()],
            configuration: vec![],
        };

        // Validate all services
        let a = a.validate().unwrap();
        let b = b.validate().unwrap();
        let c = c.validate().unwrap();

        // Create a pipeline
        let pipeline = Pipeline::new(vec![a, b, c]);
        let validated_pipeline = pipeline.validate();

        // Print all errors
        if let Err(errors) = &validated_pipeline {
            for error in errors.iter() {
                print!("{}", error);
            }
        }

        assert!(validated_pipeline.is_err());
    }

    #[test]
    fn test_valid_cyclic_pipeline() {
        // Three services, with a simple A -> B -> C dependency chain
        let a = Service {
            name: "a".to_string(),
            author: "ase-test".to_string(),
            version: "0.1.0".to_string(),
            source: "github.com/ase-test/a".to_string(),
            commands: service::Commands {
                build: None,
                run: "echo 'a'".to_string(),
            },
            inputs: vec![
                service::Input {
                    service: "b".to_string(), // depends on b
                    streams: vec!["b".to_string()],
                },
                service::Input {
                    service: "c".to_string(), // depends on c (cyclic)
                    streams: vec!["c".to_string()],
                },
            ],
            outputs: vec!["a".to_string()],
            configuration: vec![],
        };
        let b = Service {
            name: "b".to_string(),
            author: "ase-test".to_string(),
            version: "0.1.0".to_string(),
            source: "github.com/ase-test/b".to_string(),
            commands: service::Commands {
                build: None,
                run: "echo 'b'".to_string(),
            },
            inputs: vec![service::Input {
                service: "a".to_string(),
                streams: vec!["a".to_string()],
            }],
            outputs: vec!["b".to_string()],
            configuration: vec![],
        };
        let c = Service {
            name: "c".to_string(),
            author: "ase-test".to_string(),
            version: "0.1.0".to_string(),
            source: "github.com/ase-test/c".to_string(),
            commands: service::Commands {
                build: None,
                run: "echo 'c'".to_string(),
            },
            inputs: vec![service::Input {
                service: "b".to_string(),
                streams: vec!["b".to_string()],
            }],
            outputs: vec!["c".to_string()],
            configuration: vec![],
        };

        // Validate all services
        let a = a.validate().unwrap();
        let b = b.validate().unwrap();
        let c = c.validate().unwrap();

        // Create a pipeline
        let pipeline = Pipeline::new(vec![a, b, c]);
        let validated_pipeline = pipeline.validate();

        assert!(validated_pipeline.is_ok());
        // Assert that the pipeline contains the correct services
        let validated_pipeline = validated_pipeline.unwrap();
        assert_eq!(validated_pipeline.services().len(), 3);
        assert_eq!(validated_pipeline.services()[0].0.name, "a");
        assert_eq!(validated_pipeline.services()[1].0.name, "b");
        assert_eq!(validated_pipeline.services()[2].0.name, "c");
    }

    #[test]
    fn test_invalid_pipeline_missing_service() {
        // Three services, with a simple A -> B -> C dependency chain
        let a = Service {
            name: "a".to_string(),
            author: "ase-test".to_string(),
            version: "0.1.0".to_string(),
            source: "github.com/ase-test/a".to_string(),
            commands: service::Commands {
                build: None,
                run: "echo 'a'".to_string(),
            },
            inputs: vec![],
            outputs: vec!["a".to_string()],
            configuration: vec![],
        };
        let b = Service {
            name: "b".to_string(),
            author: "ase-test".to_string(),
            version: "0.1.0".to_string(),
            source: "github.com/ase-test/b".to_string(),
            commands: service::Commands {
                build: None,
                run: "echo 'b'".to_string(),
            },
            inputs: vec![service::Input {
                service: "a".to_string(),
                streams: vec!["a".to_string()],
            }],
            outputs: vec!["b".to_string()],
            configuration: vec![],
        };
        let c = Service {
            name: "c".to_string(),
            author: "ase-test".to_string(),
            version: "0.1.0".to_string(),
            source: "github.com/ase-test/c".to_string(),
            commands: service::Commands {
                build: None,
                run: "echo 'c'".to_string(),
            },
            inputs: vec![service::Input {
                service: "x".to_string(), // depends on a service that does not exist
                streams: vec!["c".to_string()],
            }],
            outputs: vec!["c".to_string()],
            configuration: vec![],
        };

        // Validate all services
        let a = a.validate().unwrap();
        let b = b.validate().unwrap();
        let c = c.validate().unwrap();

        // Create a pipeline
        let pipeline = Pipeline::new(vec![a, b, c]);
        let validated_pipeline = pipeline.validate();

        // Print all errors
        if let Err(errors) = &validated_pipeline {
            for error in errors.iter() {
                print!("{}", error);
            }
        }

        assert!(validated_pipeline.is_err());
    }

    #[test]
    fn test_invalid_pipeline_duplicate_services() {
        // Three services, with a simple A -> B -> C dependency chain
        let a = Service {
            name: "a".to_string(),
            author: "ase-test".to_string(),
            version: "0.1.0".to_string(),
            source: "github.com/ase-test/a".to_string(),
            commands: service::Commands {
                build: None,
                run: "echo 'a'".to_string(),
            },
            inputs: vec![],
            outputs: vec!["a".to_string()],
            configuration: vec![],
        };
        let b = Service {
            name: "b".to_string(),
            author: "ase-test".to_string(),
            version: "0.1.0".to_string(),
            source: "github.com/ase-test/b".to_string(),
            commands: service::Commands {
                build: None,
                run: "echo 'b'".to_string(),
            },
            inputs: vec![service::Input {
                service: "a".to_string(),
                streams: vec!["a".to_string()],
            }],
            outputs: vec!["b".to_string()],
            configuration: vec![],
        };
        let c = Service {
            name: "b".to_string(),
            author: "ase-test".to_string(),
            version: "0.1.0".to_string(),
            source: "github.com/ase-test/c".to_string(),
            commands: service::Commands {
                build: None,
                run: "echo 'c'".to_string(),
            },
            inputs: vec![service::Input {
                service: "a".to_string(),
                streams: vec!["a".to_string()],
            }],
            outputs: vec!["c".to_string()],
            configuration: vec![],
        };

        // Validate all services
        let a = a.validate().unwrap();
        let b = b.validate().unwrap();
        let c = c.validate().unwrap();

        // Create a pipeline
        let pipeline = Pipeline::new(vec![a, b, c]);
        let validated_pipeline = pipeline.validate();

        // Print all errors
        if let Err(errors) = &validated_pipeline {
            for error in errors.iter() {
                print!("{}", error);
            }
        }

        assert!(validated_pipeline.is_err());
    }

    const TEST_FILES_LOCATION: &str = "./src/testfiles/pipeline";

    #[test]
    fn test_valid_files() {
        let valid_path = format!("{}/valid", TEST_FILES_LOCATION);

        // Get all files in this directory
        let files = std::fs::read_dir(valid_path).unwrap();

        // Iterate over all files and validate them
        for file in files {
            let mut validated_services = Vec::new();

            let file = file.unwrap();
            let file_path = file.path();
            let file_name = file.file_name().into_string().unwrap();

            // Skip files
            if !file_path.is_dir() {
                continue;
            }

            println!("Validating pipeline: {}", file_name);

            // Walk over this directory and validate all files
            let service_files = std::fs::read_dir(file_path).unwrap();
            for service in service_files {
                let service = service.unwrap();
                let service_path = service.path();
                let service_name = service.file_name().into_string().unwrap();

                // Skip directories
                if service_path.is_dir() {
                    continue;
                }

                let file_content = std::fs::read_to_string(service_path).unwrap();
                let service: Service = serde_yaml::from_str(&file_content).unwrap();

                // Print errors
                if let Err(errors) = service.validate() {
                    for error in errors {
                        print!("{}\n", error);
                    }
                }

                assert!(
                    service.validate().is_ok(),
                    "Validation failed for file: {}",
                    service_name
                );

                validated_services.push(service.validate().unwrap());
            }

            // Create a pipeline
            let pipeline = Pipeline::new(validated_services);
            let validated_pipeline = pipeline.validate();

            // Print all errors
            if let Err(errors) = &validated_pipeline {
                for error in errors.iter() {
                    print!("{}", error);
                }
            }

            assert!(validated_pipeline.is_ok());
        }
    }

    #[test]
    fn test_invalid_files() {
        let invalid_path = format!("{}/invalid", TEST_FILES_LOCATION);

        // Get all files in this directory
        let files = std::fs::read_dir(invalid_path).unwrap();

        // Iterate over all files and validate them
        for file in files {
            let mut validated_services = Vec::new();

            let file = file.unwrap();
            let file_path = file.path();
            let file_name = file.file_name().into_string().unwrap();

            // Skip files
            if !file_path.is_dir() {
                continue;
            }

            println!("Validating pipeline: {}", file_name);

            // Walk over this directory and validate all files
            let service_files = std::fs::read_dir(file_path).unwrap();
            for service in service_files {
                let service = service.unwrap();
                let service_path = service.path();
                let service_name = service.file_name().into_string().unwrap();

                // Skip directories
                if service_path.is_dir() {
                    continue;
                }

                let file_content = std::fs::read_to_string(service_path).unwrap();
                let service: Service = serde_yaml::from_str(&file_content).unwrap();

                // Print errors
                if let Err(errors) = service.validate() {
                    for error in errors {
                        print!("{}\n", error);
                    }
                }

                assert!(
                    service.validate().is_ok(),
                    "Validation failed for file: {}",
                    service_name
                );

                validated_services.push(service.validate().unwrap());
            }

            // Create a pipeline
            let pipeline = Pipeline::new(validated_services);
            let validated_pipeline = pipeline.validate();

            // Print all errors
            if let Err(errors) = &validated_pipeline {
                for error in errors.iter() {
                    print!("{}", error);
                }
            }

            assert!(validated_pipeline.is_err());
        }
    }
}
