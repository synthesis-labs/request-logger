use request_logger_lib::models::RequestLoggerConfig;
use rocket::request::{FromRequest, Outcome, Request};

use crate::models::RequestContext;

#[derive(Debug)]
pub enum RequestContextError {}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestContext {
    type Error = RequestContextError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(RequestContext {
            username: "test@synthesis.co.za".to_string(),
            request_time_ms: 2,
            request_method: req.method().to_string(),
            request_uri: req.uri().to_string().to_string(),
        })
    }
}
