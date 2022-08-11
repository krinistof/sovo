use dotenv::dotenv;
use crate::schema::*;
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

    async fn check_session_by_addr(&self, addr: String) -> Option<Session> {
        Self::collection::<Session>(self, "sessions")
            .find_one(doc! {
                "address": addr
            }, None)
            .await
            .unwrap_or(None)
    }

    pub async fn create_session(&self, addr: String) -> Result<ObjectId> {
        if cfg!(feature="strict-ip-filter") {
            if let Some(doc) = Self::check_session_by_addr(self, addr.clone()).await {
                return Ok(doc._id);
            }
        }
        Self::collection(self, "sessions")
            .insert_one(doc! {
                "joined": DateTime::now(),
                "address": addr
            }, None)
            .await?
            .inserted_id
            .as_object_id()
            .context("query error")
    }
    
}
