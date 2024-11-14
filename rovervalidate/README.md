# `rovervalidate` - validate building blocks and compose pipelines

**The `rovervalidate` Rust library can be used to validate all building blocks that the ASE rover software platform uses, both validating the components *in isolation* as well as in *conjunction* with each other. With strict validation principles, users of this library that read properties from the exposed Rust structs can rest assured that they are running a pipeline that is conformant to the defined *spec*.**

For the isolated components a YAML and JSON spec can be found in the [/spec](/spec) folder. These formally define the requirements as implemented in this validation library.