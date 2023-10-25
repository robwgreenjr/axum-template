## TODO

### - Handle Error responses

### - Setup integration testing

### - Create more test to support various errors with parameter and db query builder

#### - handle panic when no query params are provided

```
thread 'tokio-runtime-worker' panicked at 'called `Option::unwrap()` on a `None` value', src/global/parameter_query_builder.rs:109:43
```

### - Support DB query builder with nested table relationships