use std::{collections::HashMap, hash::Hash};

use crate::{error::Error, START_PORT};

use rovervalidate::{pipeline::interface::RunnablePipeline, service::ValidatedService};
use serde::{Deserialize, Serialize};
use tracing::info;

use super::{service::FqBuf, DATA_ADDRESS};

#[derive(Debug, Serialize, Deserialize)]
struct Stream {
    name: String,
    address: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Input {
    service: String,
    streams: Vec<Stream>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BootSpecOutput {
    streams: Vec<Stream>,
}

#[derive(Debug, Serialize, Deserialize)]
enum BootSpecDataType {
    String(String),
    Number(f64),
}

#[derive(Debug, Serialize, Deserialize)]
struct BootSpecConfig {
    name: String,
    data_type: BootSpecDataType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BootSpec {
    name: String,
    version: String,
    inputs: Vec<Input>,
    outputs: Vec<Stream>,
    configuration: Vec<rovervalidate::service::Configuration>,
}

#[repr(transparent)]
pub struct BootSpecs(pub HashMap<FqBuf, BootSpec>);

impl BootSpecs {
    pub fn new(services: &Vec<ValidatedService>) -> Self {
        let mut start_port = START_PORT;

        let mut result = HashMap::new();

        // Create a mapping for all outputs, such that we can lookup a (service, stream)
        // and get the assigned address.
        let mut mappings: HashMap<(String, String), String> = HashMap::new();

        for validated in services {
            let s = &validated.0;
            // For each service assign an address to all of its outputs and
            // save the resulting address in the mapping.
            for out_stream in &s.outputs {
                start_port += 1;
                let address = format!("{}:{}", DATA_ADDRESS, start_port);
                let stream_name = out_stream.clone();
                mappings.insert((s.name.clone(), stream_name.clone()), address.clone());
            }
        }

        // Now that we know the mappings we can iterate over all service again
        // and set each output and input field
        for validated in services {
            let s = &validated.0;
            let service_name = &s.name;
            let fq = FqBuf::from(validated);

            let mut outputs = vec![];
            for out_stream in s.outputs.iter() {
                let stream_name = out_stream.clone();

                if let Some(address) = mappings.get(&(service_name.clone(), stream_name.clone())) {
                    outputs.push(Stream {
                        name: stream_name.clone(),
                        address: address.clone(),
                    });
                }
            }

            let mut inputs = vec![];
            for input_stream in s.inputs.iter() {
                let service_name = &input_stream.service;

                let mut streams = vec![];

                for stream_name in input_stream.streams.iter() {
                    if let Some(address) =
                        mappings.get(&(service_name.clone(), stream_name.clone()))
                    {
                        streams.push(Stream {
                            name: stream_name.clone(),
                            address: address.clone(),
                        });
                    }
                }

                inputs.push(Input {
                    service: service_name.clone(),
                    streams,
                })
            }

            let b = BootSpec {
                name: s.name.clone(),
                version: s.version.clone(),
                inputs,
                outputs,
                configuration: s.configuration.clone(),
            };

            result.insert(fq, b);
        }

        Self(result)
    }
}
