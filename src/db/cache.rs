use std::env;

use mongodb::{
    bson::oid::ObjectId,
    bson::{doc, Document},
    error::Error,
    options::FindOptions,
    Client, Collection,
};

use rocket::{
    futures::TryStreamExt,
    request::{FromRequest, Outcome},
    Request, State,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LatLong {
    pub lat: f64,
    pub lng: f64,
}

/// Full cache information
#[derive(Debug, Serialize, Deserialize)]
pub struct Cache {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub position: LatLong,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<i32>,
}

pub struct CacheDatabase {
    client: Client,
    collection: Collection<Cache>,
}

impl CacheDatabase {
    pub async fn insert_cache(&self, cache: Cache) -> Result<ObjectId, Error> {
        let inserted_id = self.collection.insert_one(cache, None).await?.inserted_id;
        Ok(inserted_id.as_object_id().unwrap())
    }

    /// Return basic cache info: position and id
    pub async fn get_caches(
        &self,
        user_id: Option<i32>,
        bounds: Option<(LatLong, LatLong)>,
    ) -> Result<Vec<Cache>, Error> {
        let mut filter = Document::new();
        if let Some(uid) = user_id {
            filter.insert("owner_id", uid);
        }

        if let Some((sw, ne)) = bounds {
            filter.insert("position.lat", doc! { "$gte": sw.lat, "$lte": ne.lat });
            filter.insert("position.lng", doc! { "$gte": sw.lng, "$lte": ne.lng });
        }

        let options = FindOptions::builder()
            .projection(doc! {
                "_id": 1,
                "position": 1,
            })
            .build();

        let cursor = self.collection.find(filter, options).await?;

        // TODO! Handle result error
        let collected: Vec<_> = cursor.try_collect().await.unwrap();
        Ok(collected)
    }

    pub async fn get_cache_by_id(&self, id: ObjectId) -> Result<Option<Cache>, Error> {
        let filter = doc! {
            "_id": id,
        };

        self.collection.find_one(filter, None).await
    }

    pub async fn update_cache(&self, cache: Cache) -> Result<(), Error> {
        let filter = doc! {
            "_id": cache.id.expect("cannot update cache withou id"),
        };

        let update = doc! {
            "$set": {
                "position.lat": cache.position.lat,
                "position.lng": cache.position.lng,
                "description": cache.description,
                "hint": cache.hint,
            },
        };

        self.collection
            .update_one(filter, update, None)
            .await
            .map(|_| ())
    }

    pub async fn delete_cache_by_id(&self, id: ObjectId) -> Result<(), Error> {
        let filter = doc! {
            "_id": id,
        };
        self.collection.delete_one(filter, None).await.map(|_| ())
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for CacheDatabase {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let client = req.guard::<&State<Client>>().await;
        let clonned = Client::clone(client.unwrap());

        let db_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
        let db = clonned.database(&db_name);
        let collection = db.collection("cache");

        Outcome::Success(Self {
            client: clonned,
            collection,
        })
    }
}
