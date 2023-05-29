#[macro_use]
extern crate rocket;

use request_logger_lib::{request_logger::{RequestLogger}, models::RequestLoggerConfig};
// use request_logger_test::{request_logger::RequestLogger, models::RequestLoggerConfig};

#[get("/test")]
async fn test() -> &'static str {
    "Ok"
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .manage(RequestLoggerConfig {
            api_url: "http://request-logger.telemetry.svc.cluster.local".to_string(),
            application_name: "test".to_string()
        })
        .attach(RequestLogger)
        .mount("/", routes![test])
}