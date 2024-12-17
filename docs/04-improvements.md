
- bug in openapi generator has private field which should be public:

``` diff
openapi::models::DuplicateServiceError(pub String);
                                       +++
```

- rovervalidate path traversal bug in commands, validation needs to error on ".."

