# roverd API spec

The spec is defined formally in apispec.yaml (using OpenAPI swagger). Using this spec, one can generate types and structs for their language. The description below is an intuition, but the spec should be used when in doubt.


# Enums
The following are Rust enums which can have an equivalent string representation in JSON. This would be useful for setting up status / state codes.

#### Process Status
``` rust
enum ProcStatus {
  Running,
  Stopped,
  Terminated,
  Killed
}
```

#### Rover Daemon Status
``` rust
enum RoverdStatus {
  Operational,
  Recoverable,
  Unrecoverable,
}
```

# API

## Processes

### `GET /procs`
``` json
[
  {
    "name": "imaging",
    "status": "Running", // String of ProcStatus
    "restarts_since": 3, // number of auto restarts since the last manual start
    "running_since": "some_timestamp", // todo agree on timestamp format unix seconds?
    "cpu": 0.2,
    "mem": 34.4, // KB?
  },
  {
    "name": "controller",
    "status": "Stopped",
    "last_run": "some_timestamp",
    "last_duration": "some_time_duration", // todo agree on format, seconds?
    "last_restarts": 3, // number of auto restarts from last run
    "cpu": 0.05,
    "mem": 14.0,
  },
  {
    "name": "imaging",
    "status": "Terminated",
    "cpu": 0.23,
    "mem": 12.8,
  },
]

```


`GET /procs/{name}?log_lines=4` - Get the status of a process, optionally add query parameter for number of log lines to receive
``` json
{
    "name": "imaging",
    "status": "Terminated",
    "pid": 123,
    "cpu": 0.23,
    "mem": 12.8,
    "logs": [
      "log line 1"
      "log line 2"
      "log line 3"
      "log line 4"
    ]
}
```

## Roverd Status

`GET /status`
``` json
{
  "status": "Operational", // RoverdStatus
  "system_time": "some_timestamp", // todo agree upon timestamp format
  "os": "Ubuntu 22.04 ...", // System information
  "version": "1.0.1" // checked at rutime
}
```

## Commands

![State Machine](./docs/StateMachine.jpg)

`GET /cmd`
``` json
[
  {
    "name": "start",
    "description": "Starts the rover given a valid configuration, builds if necessary and waits on processes."
  },
  {
    "name": "stop",
    "description": "Terminates all processes and sets to invalid state."
  },
  {
    "name": "build",
    "description": "Runs the specified build command for all services."
  },
  {
    "name": "build/{name}",
    "description": "Runs the specified build command for specified service."
  },
  {
    "name": "download",
    "description": "Fetches latest version of sources."
  },
  {
    "name": "update",
    "description": "Updates to latest version of roverd."
  },

]

```


`POST /cmd/start` - Check and validate, then fork all processes
``` json
On Success: HTTP 200
On Error:
{
  "error": "YAML Parsing error for service 'my-service'"
}
```

`POST /cmd/stop` - Stop and terminate all running procs
``` json
On Success: HTTP 200
On Error:
{
  "error": "No currently running processes"
}
```

`POST /cmd/build` - Build all services
``` json
On Success: HTTP 200
On Error:
{
  "error": "Build command failed: 'gcc: skill issue'"


  {
    "name": "imaging",
    "version": "1.0.0",
    "service_hash": "123456abcdef123456abcdef123456abcdef",
    "build_exit_code": 1,
    "build_error_message": "gcc: skill issue", // empty if build successful

  }
}
```

`POST /cmd/build/{name}` - Build specific service
``` json
On Success: HTTP 200
On Error:
{
  "name": "my_service",
  "error": "Build command failed: 'gcc: still skill issue'"
}
```

`POST /cmd/download` - Fetch latest sources and replace them.
``` json
On Success: HTTP 200
On Error:
{
  "error": "DNS Resolution failed"
}
```


`POST /cmd/update` - Self update Roverd
``` json
On Success: HTTP 200
On Error:
{
  "error": "DNS Resolution failed"
}
```

## Services
`POST /services` - Upload zip of source code
``` json
Payload: zip containing source

On Success: HTTP 200
On Error:
{
  "error": "Validation failed, dependency of service 'controller' not met"
}
```

`GET /services` - List all available services
``` json
[
  {
    "name": "imaging",
    "enabled_at": "1.0.1"
  },
  {
    "name": "controller",
    "enabled_at": "1.0.0",
  },
  {
    "name": "actuator",
    "enabled_at": "1.0.0",
  },
  {
    "name": "rpm",
  },
  {
    "name": "transceiver",
  },
]
```

`GET /services/{name}` - Get all versions of a service

``` json
{
  "name": "actuator",
  "versions": ["1.0.0", "1.0.1", "1.0.2"],
  "enabled_at": "1.0.2"
}
```


`GET /services/{name}/{version}` - Get info about a specific service
``` json
{
  "name": "actuator",
  "version": "1.0.2",
  "inputs": [
    {
      "service": "controller",
      "streams": ["steering"]
    }
  ],
  "outputs": [],
}
```

`POST /services/{name}/{version}` - Enabled/Disable a specific version, always specified with the version
``` json
// body:
{
  "enable": true,
}
```

`DELETE /services` - Delete all services
``` json
On Success: HTTP 200
On Error:
{
  "error": "Service {name} with pid ({pid}) is currently running"
}
```

`DELETE /services/{name}` - Delete all versions of specific service 
``` json
On Success: HTTP 200
On Error:
{
  "error": "Could not find service: {name}"
}
```


`DELETE /services/{name}/{version}` - Delete specific service
``` json
On Success: HTTP 200
On Error:
{
  "error": "Could not find version {version}"
}
```

## Sources

`GET /sources`
``` json
[
  {
    "name": "imaging",
    "url": "github.com/VU-ASE/imaging",
  },
  {
    "name": "controller",
    "url": "github.com/VU-ASE/controller",
  },
  {
    "name": "actuator",
    "url": "github.com/VU-ASE/actuator",
  },
]
```

`GET /sources/{url}` - Get all versions of a source
``` JSON
{
  "name": "imaging",
  "url": "github.com/VU-ASE/imaging",
  "versions": ["1.0.0", "1.4.5"]
}
```

`POST /sources` - Add a new source
``` json
Payload:
{
  "name": "imaging",
  "url": "github.com/VU-ASE/imaging",
  "version": "2.3.4",
  "sha": "1234"
}

Response:
On Success: HTTP 200
On Error:
{
  "error": "Source {url} with version {version} already exists"
}
```


`DELETE /sources/{url}` - Delete a source
``` json
{
  "name": "imaging",
  "url": "github.com/VU-ASE/imaging",
  "version": "2.3.4",
  "sha": "1234"
}

Response:
On Success: HTTP 200
On Error:
{
  "error": "No such source {url} with version {version}"
}
```

# Paths on Rover
* Services: `/home/debix/.rover/{name}/{version}/service.yaml`
* State: `/etc/rover/rover.yaml`
