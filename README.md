# Request Logger

A Rust Rocket request logger [fairing (middleware)](https://rocket.rs/v0.5-rc/guide/fairings/)

```toml
[dependencies]
request_logger_lib = { version = "0.2.0", git = "https://github.com/synthesis-labs/request-logger.git" }
```

```rust
use request_logger_lib::request_logger::{RequestLogger};
...
rocket::build()
    .manage(RequestLoggerConfig {
        api_url: "http://request-logger.telemetry.svc.cluster.local".to_string(),
        application_name: "<APP>".to_string()
    })
    .attach(RequestLogger)
    .mount("/", routes![index])

```

Updating the dependency within a project:

```sh
$ cargo update -p request_logger_lib
```

# Database (Prisma)

First define a model in `prisma/schema.prisma`

```prisma
datasource db {
  provider = "postgresql"
  url      = "postgresql://user:password@host:5432/database"
}

generator client {
  provider = "cargo prisma"
  output   = "../src/prisma.rs"
}

model http_requests {
  id              String @id @default(cuid())
  time_generated  DateTime @default(now())
}
```

Then regenerate the client (`src/prisma.rs`):

```sh
$ cargo prisma generate
```

Finally generate migrations

```sh
$ cargo prisma migrate dev
```

## TODO
- Get JWT subject
- Provide function to Fairing
- Also pop to S3