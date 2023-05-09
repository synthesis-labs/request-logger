use std::time::SystemTime;
use rocket::{fairing::{Fairing, Info, Kind}, Request, Response, Data, request::{FromRequest, self}, http::Status};

pub struct RequestTimer;

#[derive(Copy, Clone)]
struct TimerStart(Option<SystemTime>);

#[rocket::async_trait]
impl Fairing for RequestTimer {
    fn info(&self) -> Info {
        Info {
            name: "Request Timer",
            kind: Kind::Request | Kind::Response
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        request.local_cache(|| TimerStart(Some(SystemTime::now())));
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        let start_time = req.local_cache(|| TimerStart(None));
        if let Some(Ok(duration)) = start_time.0.map(|st| st.elapsed()) {
            let ms = duration.as_secs() * 1000 + duration.subsec_millis() as u64;
            res.set_raw_header("X-Response-Time", format!("{} ms", ms));
        }
    }
}

#[derive(Copy, Clone)]
pub struct StartTime(pub SystemTime);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for StartTime {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, ()> {
        match *request.local_cache(|| TimerStart(None)) {
            TimerStart(Some(time)) => request::Outcome::Success(StartTime(time)),
            TimerStart(None) => request::Outcome::Failure((Status::InternalServerError, ())),
        }
    }
}