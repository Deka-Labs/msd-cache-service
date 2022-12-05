use std::env;

use mongodb::Client;
use rocket::serde::json::Json;
use rocket::{Build, Rocket};

mod cache;
pub use cache::Cache;
pub use cache::CacheDatabase;
pub use cache::LatLong;
use serde_json::{json, Value};

#[async_trait]
pub trait RockerDatabaseConnect {
    async fn connect_database(self) -> Self;
}

#[async_trait]
impl RockerDatabaseConnect for Rocket<Build> {
    async fn connect_database(self) -> Self {
        let db_path = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let client = Client::with_uri_str(&db_path).await.unwrap();

        // Just check for other calls
        let _ = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");

        self.manage(client)
    }
}

#[derive(Debug, Responder)]
#[response(status = 500)]
pub struct DatabaseErrorResponse(Json<Value>);
impl DatabaseErrorResponse {
    pub fn new(error: mongodb::error::Error) -> Self {
        Self(Json(json!({
            "message": "Database error",
            "database_error": format!("{:?}", error)
        })))
    }
}
