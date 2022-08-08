use dotenv::dotenv;
//use futures::TryStreamExt;
use std::{env, io::Error};

use mongodb::{
    bson::{doc, oid::ObjectId},
    Client, Collection, Database,
};

pub struct Mongo {
    db: Database,
}

impl Mongo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = env::var("MONGOURI")
            .expect("error loading env variable");
        let client = Client::with_uri_str(uri)
            .await
            .expect("error connecting to database");
        let db = client.database("projectMngt");
        Mongo { db }
    }

    fn col_helper<T>(data_source: &Self, collection_name: &str) -> Collection<T> {
        data_source.db.collection(collection_name)
    }

    //
}
