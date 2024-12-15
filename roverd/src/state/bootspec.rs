use std::collections::HashMap;

use crate::error::Error;

use rovervalidate::{pipeline::interface::RunnablePipeline, service::ValidatedService};
use serde::{Deserialize, Serialize};

use super::DATA_ADDRESS;

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

impl BootSpec {
    pub fn new(start_port: &mut u32, service: &ValidatedService) -> Self {
        let s = &service.0;

        // Create a mapping for all outputs, such that we can lookup a (service, stream)
        // and get the assigned address.
        let mut mappings: HashMap<(String, String), String> = HashMap::new();

        // First, for each service assign an address to all of its outputs and
        // save the resulting address in the mapping.
        let outputs = s
            .outputs
            .iter()
            .map(|out_stream| {
                *start_port += 1;
                let address = format!("{}:{}", DATA_ADDRESS, start_port);
                let stream_name = out_stream.clone();

                mappings.insert((s.name.clone(), stream_name.clone()), address.clone());

                Stream {
                    name: stream_name,
                    address,
                }
            })
            .collect::<Vec<Stream>>();

        // Now that we have the mapping, for each service's inputs we lookup the address
        let inputs = s
            .inputs
            .iter()
            .map(|input| {
                let service_name = input.service.clone();
                let mut streams: Vec<Stream> = vec![];

                for stream_name in input.streams.iter() {
                    if let Some(address) =
                        mappings.get(&(service_name.clone(), stream_name.clone()))
                    {
                        streams.push(Stream {
                            name: stream_name.clone(),
                            address: address.clone(),
                        });
                    }
                }
                Input {
                    service: service_name,
                    streams: streams,
                }
            })
            .collect::<Vec<Input>>();

        BootSpec {
            name: s.name.clone(),
            version: s.version.clone(),
            inputs,
            outputs,
            configuration: s.configuration.clone(),
        }
    }
}
