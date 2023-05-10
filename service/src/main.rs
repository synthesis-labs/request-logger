#[allow(warnings, unused)]
mod prisma;

use prisma::{http_requests, PrismaClient};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate rocket;

// #[get("/")]
// async fn index() -> &'static str {
//   let client = PrismaClient::_builder().build().await.unwrap();

//   let _http_request: http_requests::Data = client
//       .http_requests()
//       .create(200, vec![
//           http_requests::app::set("galactica".to_string()),
//           http_requests::username::set("tjaard@synthesis.co.za".to_string()),
//           http_requests::request_method::set("GET".to_string()),
//           http_requests::request_uri::set("api/something".to_string()),
//           // http_requests::request_body::set("galactica".to_string()),
//           // http_requests::response_body::set("galactica".to_string()),
//       ])
//       .exec()
//       .await
//       .unwrap();

//     "Ok"
// }

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

#[post("/metric", data = "<input>")]
async fn metric(input: Json<MetricRequest>) -> &'static str {
    let client = PrismaClient::_builder().build().await.unwrap();

    let _http_request: http_requests::Data = client
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
    rocket::build().mount("/", routes![metric])
}
