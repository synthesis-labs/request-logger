# Request Logger

A Rust Rocket request logger [fairing (middleware)](https://rocket.rs/v0.5-rc/guide/fairings/)

```toml
[dependencies]
request-logger = { git = "https://github.com/synthesis-labs/request-logger.git" }
```

```rust
use request_logger::timer_fairing::{RequestTimer};
...
rocket::build().attach(RequestTimer).mount("/", routes![index])
```


