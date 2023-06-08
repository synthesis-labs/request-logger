use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetricRequest {
    pub app: String,
    pub username: String,
    pub request_time_ms: i32,
    pub request_method: String,
    pub request_uri: String,
}

pub struct RequestLoggerConfig {
    pub api_url: String,
    pub application_name: String
}

pub struct RequestContext {
    pub username: String,
    pub request_time_ms: i32,
    pub request_method: String,
    pub request_uri: String,
}