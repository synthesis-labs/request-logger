#[allow(warnings, unused)]
mod prisma;

use std::sync::Arc;

use prisma::http_requests;
use rocket::serde::json::Json;
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
    // pub time_generated  : DateTime ,
    pub username: String,
    pub request_time_ms: i32,
    pub request_method: String,
    pub request_uri: String,
    pub request_body: String,
    pub response_body: String,
}

#[get("/")]
async fn index() -> &'static str {
    "Ok"
}

#[post("/metric", data = "<input>")]
async fn metric(input: Json<MetricRequest>, ctx: &Ctx) -> &'static str {
    // let client = PrismaClient::_builder().build().await.unwrap();

    let _http_request: http_requests::Data = ctx
        .db
        .http_requests()
        .create(
            input.request_time_ms,
            vec![
                http_requests::app::set(input.app.clone()),
                http_requests::username::set(input.username.clone()),
                http_requests::request_method::set(input.request_method.clone()),
                http_requests::request_uri::set(input.request_uri.clone()),
                http_requests::request_body::set(input.request_body.clone()),
                http_requests::response_body::set(input.response_body.clone()),
            ],
        )
        .exec()
        .await
        .unwrap();

    "Lekker..."
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
