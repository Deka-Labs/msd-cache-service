#[macro_use]
extern crate rocket;
extern crate mongodb;

use rocket::{serde::json::Json, Request};

mod cors;
use cors::RocketCorsEnabler;

mod db;
use db::RockerDatabaseConnect;

mod routes;
use routes::RocketRoutesAdd;

mod status;
use status::ResponseError;

mod auth;
mod login_service;

#[catch(404)]
pub fn not_found_catcher(req: &Request) -> Json<ResponseError> {
    let err_msg = format!(
        "URL: '{}' not found for method {}",
        req.uri().path().as_str(),
        req.method().as_str()
    );
    Json(ResponseError::new(err_msg))
}

#[catch(500)]
pub fn unhandled_catcher(req: &Request) -> Json<ResponseError> {
    let err_msg = format!(
        "There are unhandled error in URL '{}' for method {}.
         Contact a support. With folowing message but make sure to delete keys, password, etc from it:
         REQUEST:
         {}",
        req.uri().path().as_str(),
        req.method().as_str(),
        req,
    );
    Json(ResponseError::new(err_msg))
}

#[launch]
async fn rocket() -> _ {
    // Load .env
    dotenvy::dotenv().ok();

    let api_base = "/api/v1";

    rocket::build()
        .enable_cors()
        .connect_database()
        .await
        .register(api_base, catchers![not_found_catcher, unhandled_catcher])
        .routes_add(api_base)
}
