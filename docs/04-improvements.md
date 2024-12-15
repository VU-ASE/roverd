
- bug in openapi generator has private field which should be public:

``` diff
openapi::models::DuplicateServiceError(pub String);
                                       +++
```



