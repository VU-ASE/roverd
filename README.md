# roverd

Roverd is an always running process on the rover (daemon) which exposes endpoints that allow programs like `roverctl` to interact with the rover. This repo also defines the API specification which clients need to implement in order to use the provided functionality. In short, roverd lets you view system status, upload services and start/stop a collection of services (a pipeline).


## Development

All dependencies are bundled in the devcontainer, as well as a debix user and filesystem setup identical to that of a rover. Run `make dev` for development. If changes are made to [`apispec.yaml`](/roverd/spec/apispec.yaml), then the openapi definitions must be generated again with `cd roverd ; make build`.

> Important: due to bugs in the openapi generator, some tuple structs have private members, which needs to be updated manually after re-generating the openapi definitions. After running `make build` the following need to be edited by hand after which everything should compile.

[`roverd/openapi/src/models.rs`](/roverd/openapi/src/models.rs)
```
                                 +++
pub struct DuplicateServiceError(pub String);

                                                                             +++
pub struct ServicesAuthorServiceVersionGet200ResponseConfigurationInnerValue(pub Box<serde_json::value::RawValue> );
```

For interacting with the API, the Swagger extension (already installed through devcontainer) is extremely helpful. It lets you test authorized API requests based on the specification.

## Directories
* `/roverd` - source code for roverd
* `/roverd/spec` - openapi and bootspec specifications
* `/roverd/example-pipelines` - dummy services that can be used for testing
* `/roverd/openapi` - generated rust code from openapi
* `/rovervalidate` - library that performs validation of service and configuration files
* `/scripts` - useful for testing


## Future Improvements
This repo has an unnecessarily **large** amount of code due to type conversions between types generated from openapi and validation types in rovervalidate. Furthermore, boilerplate code could be largely reduced by not generating Rust code from openapi, but by generating a openapi defintion form Rust types.

