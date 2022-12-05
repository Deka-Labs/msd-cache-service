use crate::{
    auth::AuthInfo,
    db::{CacheDatabase, DatabaseErrorResponse},
    status::ResponseError,
};
use mongodb::bson::oid::ObjectId;
use rocket::serde::json::{Json, Value};
use serde_json::json;

pub enum DeleteResult {
    Ok,
    DBError(mongodb::error::Error),
    WrongObjectID,
}

#[derive(Debug, Responder)]
pub enum DeleteResultResponse {
    #[response(status = 200)]
    Ok(Json<Value>),
    DBError(DatabaseErrorResponse),

    #[response(status = 400)]
    WrongObjectID(Json<ResponseError>),
}

impl From<DeleteResult> for DeleteResultResponse {
    fn from(res: DeleteResult) -> Self {
        match res {
            DeleteResult::Ok => Self::Ok(Json(json!({}))),
            DeleteResult::DBError(err) => Self::DBError(DatabaseErrorResponse::new(err)),
            DeleteResult::WrongObjectID => Self::WrongObjectID(Json(ResponseError::new(
                "Wrong ObjectID format".to_string(),
            ))),
        }
    }
}

#[delete("/<id>")]
pub async fn delete_cache(
    id: String,
    cache_db: CacheDatabase,
    _auth: AuthInfo,
) -> DeleteResultResponse {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return DeleteResult::WrongObjectID.into();
    };

    match cache_db.delete_cache_by_id(oid).await {
        Ok(_) => DeleteResult::Ok.into(),
        Err(err) => DeleteResult::DBError(err).into(),
    }
}
