#[allow(warnings, unused)]
mod prisma;

use std::sync::Arc;
use prisma::http_requests::{self};
use rocket::{serde::json::Json, http::Status, response::status::{BadRequest, Accepted}};
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate rocket;

#[derive(Clone)]
pub struct Context {
    pub db: Arc<prisma::PrismaClient>,
}

pub type Ctx = rocket::State<Context>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetricRequest {
    pub app: String,
    pub username: String,
    pub request_time_ms: i32,
    pub request_method: String,
    pub request_uri: String,
}

#[get("/")]
async fn index() -> Status {
    Status::Ok
}

#[post("/metric", data = "<input>")]
async fn metric(input: Json<MetricRequest>, ctx: &Ctx) -> Result<Accepted<()>, BadRequest<String>> {
    let http_request = ctx
        .db
        .http_requests()
        .create(
            input.request_time_ms,
            vec![
                http_requests::app::set(input.app.clone()),
                http_requests::username::set(input.username.clone()),
                http_requests::request_method::set(input.request_method.clone()),
                http_requests::request_uri::set(input.request_uri.clone()),
            ],
        )
        .exec()
        .await;

    let response = match http_request {
        Ok(_) => Ok(Accepted(None)),
        Err(e) => Err(BadRequest(Some(e.to_string()))),
    };

    response
}

#[launch]
async fn rocket() -> _ {
    let db = Arc::new(
        prisma::new_client()
            .await
            .expect("Failed to create Prisma client"),
    );

    rocket::build()
        .manage(Context { db })
        .mount("/", routes![index, metric])
}
