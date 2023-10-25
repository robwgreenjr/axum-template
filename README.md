## TODO

### - Handle Error responses

### - Setup integration testing

### - Create more test to support various errors with parameter and db query builder

#### - handle panic when no query params are provided

```
thread 'tokio-runtime-worker' panicked at 'called `Option::unwrap()` on a `None` value', src/global/parameter_query_builder.rs:109:43
```

### - Support DB query builder with nested table relationships

### - See how events work (implement for user create/update/delete)

### - Complete user module (finalize basic architecture for a module)

### - Port Hypermedia

### - Port Authentication & Authorization

### - Port AWS basics (only SES for now)

### - Setup Migration plan

### - Create cli tool for basic things e.g create admin user, reset password, and migration/seeder if needed

### - Document query parameter builder