use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetricRequest {
    pub app: String,
    // pub time_generated  : DateTime ,
    pub username: String,
    pub request_time_ms: i32,
    pub request_method: String,
    pub request_uri: String,
    pub request_body: String,
    pub response_body: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetricResponse {
    pub error: String
}