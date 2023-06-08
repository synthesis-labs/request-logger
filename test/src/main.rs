#[macro_use]
extern crate rocket;

use request_logger_test::models::RequestContext;

use request_logger_lib::{request_logger::RequestLogger, models::RequestLoggerConfig};
// use request_logger_test::{request_logger::RequestLogger, models::RequestLoggerConfig};

#[get("/test")]
fn test(requestCtx: RequestContext) -> &'static str {
    RequestLogger.log(requestCtx, "{ 'instruction': 'something' }", "{ 'response': 'something' }");
    "Ok"
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .manage(RequestLoggerConfig {
            api_url: "http://request-logger.telemetry.svc.cluster.local".to_string(),
            application_name: "test".to_string()
        })
        .mount("/", routes![test])
}