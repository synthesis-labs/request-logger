use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetricRequest {
    pub app: String,
    pub username: String,
    pub request_time_ms: i32,
    pub request_method: String,
    pub request_uri: String,
}

#[tokio::main]
async fn main() {
    // let client = reqwest::Client::new();

    // let response = client
    //     .get("http://localhost:8000/fail")
    //     .send()
    //     .await;

    // match response {
    //     Ok(r) => match r.error_for_status() {
    //         Ok(d) => println!("success"),
    //         Err(e) => eprintln!("{e}")
    //     },
    //     Err(e) => eprintln!("{e}")
    // }

    let metric = MetricRequest {
        app: "test".to_string(),
        username: "test@synthesis.co.za".to_string(),
        request_time_ms: 200,
        request_method: "get".to_string(),
        request_uri: "test".to_string(),
    };

    let client = reqwest::Client::new();

    let response = client
        .post(format!("http://localhost:8000/metric"))
        .json(&metric)
        .send()
        .await;

    match response {
        Ok(r) => match r.error_for_status() {
            Ok(_) => println!("request_logger: Successfully logged request"),
            Err(e) => eprintln!("request_logger error: {e}")
        },
        Err(e) => eprintln!("request_logger error: {e}")
    }
}
