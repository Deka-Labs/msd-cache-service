use std::env;

use mongodb::Client;
use rocket::{Build, Rocket};

mod cache;

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
