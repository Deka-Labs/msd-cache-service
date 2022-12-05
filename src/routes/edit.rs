use mongodb::bson::oid::ObjectId;
use rocket::serde::json::Json;
use serde_json::{json, Value};

use crate::{
    auth::AuthInfo,
    db::{Cache, CacheDatabase, DatabaseErrorResponse},
    status::ResponseError,
};

#[derive(Debug, Responder)]
pub struct CacheEditResponse(Json<Value>);
impl CacheEditResponse {
    pub fn new() -> Self {
        Self(Json(json!({})))
    }
}

pub enum CacheEditError {
    WrongObjectID,
    DBError(mongodb::error::Error),
}

#[derive(Debug, Responder)]
pub enum CacheEditErrorResponse {
    #[response(status = 400)]
    WrongObjectID(Json<ResponseError>),

    DBError(DatabaseErrorResponse),
}

impl From<CacheEditError> for CacheEditErrorResponse {
    fn from(err: CacheEditError) -> Self {
        match err {
            CacheEditError::WrongObjectID => Self::WrongObjectID(Json(ResponseError::new(
                "Wrong ObjectID format".to_string(),
            ))),
            CacheEditError::DBError(err) => Self::DBError(DatabaseErrorResponse::new(err)),
        }
    }
}

#[put("/<id>", format = "json", data = "<cache>")]
pub async fn edit_cache(
    id: String,
    cache: Json<Cache>,
    cache_db: CacheDatabase,
    _auth: AuthInfo,
) -> Result<CacheEditResponse, CacheEditErrorResponse> {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return Err(CacheEditError::WrongObjectID.into());
    };

    let mut cache_new = cache.0;
    cache_new.id = Some(oid);

    match cache_db.update_cache(cache_new).await {
        Ok(_) => Ok(CacheEditResponse::new()),
        Err(err) => Err(CacheEditError::DBError(err).into()),
    }
}
