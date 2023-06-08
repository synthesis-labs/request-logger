use crate::models::{MetricRequest, RequestLoggerConfig, RequestContext};


async fn log(ctx: RequestContext, request_data_json: String, response_data_json: String) {
    let default_config: RequestLoggerConfig = RequestLoggerConfig {
        api_url: "request-logger.telemetry.svc.cluster.local".to_string(),
        application_name: "N/A".to_string(),
    };

    let config = match req.rocket().state::<RequestLoggerConfig>() {
        Some(cnf) => cnf,
        None => &default_config,
    };

    let metric = MetricRequest {
        app: config.application_name.to_string(),
        username: ctx.username,
        request_time_ms: ctx.request_time_ms,
        request_method: ctx.request_method,
        request_uri: ctx.request_uri,
        // request_data_json,
        // response_data_json
    };

    let client = reqwest::Client::new();

    let response = client
        .post(format!("{}/metric", config.api_url))
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
