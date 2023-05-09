# Request Logger

A Rust Rocket request logger [fairing (middleware)](https://rocket.rs/v0.5-rc/guide/fairings/)

```rust
use request_logger::timer_fairing::{RequestTimer};
...
rocket::build().attach(RequestTimer).mount("/", routes![index])
```


