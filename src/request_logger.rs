// #[allow(warnings, unused)]
// mod prisma;

use std::time::SystemTime;
use crate::prisma::http_requests;
use crate::prisma::PrismaClient;
use rocket::{
    fairing::{Fairing, Info, Kind},
    Data, Request, Response,
};

pub struct RequestLogger;

#[derive(Copy, Clone)]
struct TimerStart(Option<SystemTime>);

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
        let start_time = req.local_cache(|| TimerStart(None));
        if let Some(Ok(duration)) = start_time.0.map(|st| st.elapsed()) {
            let ms = duration.as_secs() as i32 * 1000 + duration.subsec_millis() as i32;

            let client = PrismaClient::_builder().build().await.unwrap();

            let _http_request: http_requests::Data = client
                .http_requests()
                .create(ms, vec![
                    http_requests::app::set("galactica".to_string()),
                    http_requests::username::set("tjaard@synthesis.co.za".to_string()),
                    http_requests::request_method::set(req.method().to_string()),
                    http_requests::request_uri::set(req.uri().to_string()),
                    // http_requests::request_body::set("galactica".to_string()),
                    // http_requests::response_body::set("galactica".to_string()),
                ])
                .exec()
                .await
                .unwrap();
        }
    }
}
