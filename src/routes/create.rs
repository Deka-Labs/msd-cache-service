use crate::{
    auth::AuthInfo,
    db::{Cache, CacheDatabase, DatabaseErrorResponse},
};
use mongodb::bson::oid::ObjectId;
use rocket::serde::json::{Json, Value};
use serde_json::json;

#[derive(Responder)]
#[response(status = 201)]
pub struct CacheAdded(Json<Value>);
impl CacheAdded {
    pub fn new(id: ObjectId) -> Self {
        Self(Json(json! ({
            "id": id,
        })))
    }
}

pub enum CacheError {
    DatabaseError(mongodb::error::Error),
}

#[derive(Debug, Responder)]
pub enum CacheErrorResponse {
    DBError(DatabaseErrorResponse),
}

impl From<CacheError> for CacheErrorResponse {
    fn from(err: CacheError) -> Self {
        match err {
            CacheError::DatabaseError(db_err) => Self::DBError(DatabaseErrorResponse::new(db_err)),
        }
    }
}

#[post("/", format = "json", data = "<cache>")]
pub async fn create_cache(
    cache: Json<Cache>,
    cache_db: CacheDatabase,
    auth: AuthInfo,
) -> Result<CacheAdded, CacheErrorResponse> {
    // Set user id as owner
    let mut cache_to_add = cache.0;
    cache_to_add.owner_id = Some(auth.user_id);

    match cache_db.insert_cache(cache_to_add).await {
        Ok(id) => Ok(CacheAdded::new(id)),
        Err(e) => Err(CacheError::DatabaseError(e).into()),
    }
}
