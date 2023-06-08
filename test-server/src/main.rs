
#[macro_use]
extern crate rocket;

use rocket::{response::status::{Accepted, BadRequest}, serde::json::Json};

#[get("/accepted")]
async fn accepted() -> Accepted<()> {
    Accepted(None)
}

#[get("/json")]
async fn json() -> Json<String> {
    Json(String::from("hello"))
}

#[get("/fail")]
async fn fail() -> BadRequest<String> {
    BadRequest(Some(String::from("rompelstompel")))
}

#[get("/result")]
async fn result() -> Result<Accepted<()>, BadRequest<String>> {
    Err(BadRequest(Some(String::from("result error"))))
    // Ok(Accepted(None))
}


#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![accepted, json, fail, result])
}