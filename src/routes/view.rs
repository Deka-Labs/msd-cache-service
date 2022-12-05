use mongodb::bson::oid::ObjectId;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::{
    db::{Cache, CacheDatabase, DatabaseErrorResponse, LatLong},
    status::ResponseError,
};

#[derive(Serialize, Deserialize, FromForm)]
pub struct CacheViewParameters {
    pub user_id: Option<i32>,

    pub min_lat: Option<f64>,
    pub max_lat: Option<f64>,

    pub min_long: Option<f64>,
    pub max_long: Option<f64>,
}

impl CacheViewParameters {
    pub fn coordinates_provided(&self) -> bool {
        self.min_lat.is_some()
            || self.max_lat.is_some()
            || self.min_long.is_some()
            || self.max_long.is_some()
    }

    pub fn coordinates_complete(&self) -> bool {
        self.min_lat.is_some()
            && self.max_lat.is_some()
            && self.min_long.is_some()
            && self.max_long.is_some()
    }

    pub fn get_bound_points(&self) -> Option<(LatLong, LatLong)> {
        if !self.coordinates_provided() {
            return None;
        }

        if !self.coordinates_complete() {
            return None;
        }

        let mut min_point = LatLong {
            lat: self.min_lat.unwrap(),
            lng: self.min_long.unwrap(),
        };

        let mut max_point = LatLong {
            lat: self.max_lat.unwrap(),
            lng: self.max_long.unwrap(),
        };

        if self.min_lat > self.max_lat {
            std::mem::swap(&mut min_point.lat, &mut max_point.lat);
        }

        if self.min_long > self.max_long {
            std::mem::swap(&mut min_point.lng, &mut max_point.lng);
        }

        Some((min_point, max_point))
    }
}

#[derive(Debug, Serialize)]
pub struct CacheView {
    caches: Vec<Cache>,
}

#[derive(Debug, Responder)]
pub struct CacheViewResponse(Json<CacheView>);

impl From<CacheView> for CacheViewResponse {
    fn from(v: CacheView) -> Self {
        Self(Json(v))
    }
}

pub enum CacheViewErrors {
    IncompleteBoundSpecification,
    DatabaseError(mongodb::error::Error),
}

#[derive(Debug, Responder)]
pub enum CacheViewErrorResponse {
    #[response(status = 400)]
    IncompleteBoundSpecification(Json<ResponseError>),
    DBError(DatabaseErrorResponse),
}

impl From<CacheViewErrors> for CacheViewErrorResponse {
    fn from(err: CacheViewErrors) -> Self {
        match err {
            CacheViewErrors::IncompleteBoundSpecification => {
                Self::IncompleteBoundSpecification(Json(ResponseError::new(
                    "Необходимо задать все границы области поиска".to_string(),
                )))
            }
            CacheViewErrors::DatabaseError(e) => Self::DBError(DatabaseErrorResponse::new(e)),
        }
    }
}

#[get("/?<params..>")]
pub async fn view_caches(
    params: CacheViewParameters,
    cache_db: CacheDatabase,
) -> Result<CacheViewResponse, CacheViewErrorResponse> {
    let bounds = params.get_bound_points();
    // If coords provided but we cannot create bounds it means that not all coordiantes provided
    if params.coordinates_provided() && bounds.is_none() {
        return Err(CacheViewErrors::IncompleteBoundSpecification.into());
    }

    match cache_db.get_caches(params.user_id, bounds).await {
        Ok(caches) => Ok(CacheView { caches }.into()),
        Err(err) => Err(CacheViewErrors::DatabaseError(err).into()),
    }
}

#[get("/<id>")]
pub async fn view_cache(id: String, cache_db: CacheDatabase) -> Option<CacheViewResponse> {
    let Ok(oid) = ObjectId::parse_str(&id) else {
        return None;
    };

    match cache_db.get_cache_by_id(oid).await {
        Ok(Some(c)) => Some(CacheView { caches: vec![c] }.into()),
        Err(_) => None,
        _ => None,
    }
}
