use dotenv::dotenv;
use crate::schema::*;
//use futures::TryStreamExt;
use std::env;
use anyhow::{Result, Context};
use mongodb::{
    bson::{doc, oid::ObjectId, DateTime},
    Client, Collection, Database,
};

pub struct Mongo {
    db: Database,
}

impl Mongo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = env::var("MONGODB_URI")
            .expect("error loading env variable");
        let client = Client::with_uri_str(uri)
            .await
            .expect("error connecting to database");
        let db = client.database("sovo");
        Mongo { db }
    }

    fn collection<T>(data_source: &Self, collection_name: &str) -> Collection<T> {
        data_source.db.collection(collection_name)
    }

    pub async fn create_session(&self) -> Result<ObjectId> {
        Self::collection(self, "sessions")
            .insert_one(doc! {
                "joined": DateTime::now()
            }, None)
            .await?
            .inserted_id
            .as_object_id()
            .context("query error")
    }
    
}
