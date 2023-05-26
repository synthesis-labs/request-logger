use std::time::SystemTime;
use rocket::{
    fairing::{Fairing, Info, Kind},
    Data, Request, Response
};

use crate::models::{MetricRequest, MetricResponse, RequestLoggerConfig};

pub struct RequestLogger;

#[derive(Copy, Clone)]
struct TimerStart(Option<SystemTime>);

async fn post_metric(
    api_url: String,
    metric: MetricRequest,
) -> Result<MetricResponse, reqwest::Error> {
    let client = reqwest::Client::new();

    let response = client
        .post(format!("{api_url}/metric"))
        .json(&metric)
        .send()
        .await?
        .json::<MetricResponse>()
        .await;

    return response;
}

#[rocket::async_trait]
impl Fairing for RequestLogger {
    fn info(&self) -> Info {
        Info {
            name: "Request Logger",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        request.local_cache(|| TimerStart(Some(SystemTime::now())));
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        let default_config = RequestLoggerConfig {
            api_url: "request-logger.telemetry.svc.cluster.local".to_string(),
            application_name: "N/A".to_string(),
        };

        let config = match req.rocket().state::<RequestLoggerConfig>() {
            Some(cnf) => cnf,
            None => &default_config,
        };

        let start_time = req.local_cache(|| TimerStart(None));
        if let Some(Ok(duration)) = start_time.0.map(|st| st.elapsed()) {
            let ms = duration.as_secs() as i32 * 1000 + duration.subsec_millis() as i32;

            let metric = MetricRequest {
                app: config.application_name.to_string(),
                username: "test@synthesis.co.za".to_string(),
                request_time_ms: ms,
                request_method: req.method().to_string(),
                request_uri: req.uri().to_string(),
                request_body: "{}".to_string(),
                response_body: "{}".to_string(),
            };

            let response = post_metric(config.api_url.to_string(), metric.clone()).await;

            match response {
                Ok(_) => println!("request_logger: Successfully logged request"),
                Err(err) => eprintln!("request_logger error: {err}"),
            };
        }
    }
}
