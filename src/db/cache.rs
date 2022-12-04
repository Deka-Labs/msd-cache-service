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

    pub description: Option<String>,
    pub hint: Option<String>,

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
            filter.insert(
                "position",
                doc! {
                    "lat": { "$gte": sw.lat, "$lte": ne.lat },
                    "lng": { "$gte": sw.lng, "$lte": ne.lng }
                },
            );
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
