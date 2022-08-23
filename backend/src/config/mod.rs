use crate::schema::{Opinion, Party, Session};
use anyhow::anyhow;
use anyhow::{Context, Result};
use dotenv::dotenv;
use futures_util::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, Bson::Null, DateTime, Document},
    options::UpdateOptions,
    Client, Collection, Database,
};
use std::env;

pub struct Mongo {
    db: Database,
}

impl Mongo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = env::var("MONGODB_URI").expect("error loading env variable");
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
            .find_one(
                doc! {
                    "address": addr
                },
                None,
            )
            .await
            .unwrap_or(None)
    }

    pub async fn create_session(&self, addr: String) -> Result<ObjectId> {
        if cfg!(feature = "strict-ip-filter") {
            if let Some(doc) = Self::check_session_by_addr(self, addr.clone()).await {
                return Ok(doc._id);
            }
        }
        Self::collection(self, "sessions")
            .insert_one(
                doc! {
                    "joined": DateTime::now(),
                    "address": addr
                },
                None,
            )
            .await?
            .inserted_id
            .as_object_id()
            .context("query error")
    }

    pub async fn create_party(&self, partyid: String, password: String) -> Result<()> {
        Self::collection(self, "parties")
            .insert_one(
                doc! {
                    "_id": partyid,
                    "isLive": false,
                    "currentSong": Null,
                    "password": password,
                    "queue": [],
                },
                None,
            )
            .await?;
        Ok(())
    }

    pub async fn process_vote(
        &self,
        session: ObjectId,
        partyid: String,
        song_oid: ObjectId,
        opinion: Opinion,
    ) -> Result<()> {
        let parties = Self::collection::<Party>(self, "parties");

        // remove previous vote if any
        parties
            .update_one(
                doc! {
                    "_id": &partyid
                },
                doc! {
                    "$pull": { "queue.$[element].votes": {
                            "voter": session
                        }
                    }
                },
                UpdateOptions::builder()
                    .array_filters(vec![doc! {"element.songid": song_oid}])
                    .build(),
            )
            .await?;

        if let Opinion::Neutral = opinion {
            return Ok(());
        }

        // update song with vote
        parties
            .update_one(
                doc! {
                    "_id": &partyid
                },
                doc! {
                    "$push": {"queue.$[element].votes" : {
                        "voter": session,
                        "opinion": opinion.int()
                    }}
                },
                UpdateOptions::builder()
                    .array_filters(vec![doc! {"element.songid": song_oid}])
                    .build(),
            )
            .await?;

        // order queue
        Self::sort_by_rank(self, &partyid).await?;
        Ok(())
    }

    async fn calculate_rank(&self, partyid: &str) -> Result<()> {
        Self::collection::<Party>(self, "parties")
            .update_one(
                doc! {
                    "_id": partyid
                },
                vec![doc! {
                    "$addFields": {
                        "queue": {
                            "$map": {
                                "input": "$queue",
                                "in": {"$mergeObjects": ["$$this", {
                                    "rank": {"$sum": "$$this.votes.opinion"}
                                }]}
                            }
                        }
                    }
                }],
                None,
            )
            .await?;
        Ok(())
    }

    async fn sort_by_rank(&self, partyid: &str) -> Result<()> {
        Self::calculate_rank(self, partyid).await?;

        Self::collection::<Party>(self, "parties")
            .update_one(
                doc! { "_id": partyid },
                doc! {
                    "$push": {
                        "queue": {
                            "$each": [],
                            "$sort": {"rank": -1}
                        }
                    }
                },
                None,
            )
            .await?;
        Ok(())
    }

    pub async fn add_propose(
        &self,
        session: ObjectId,
        partyid: String,
        songid: ObjectId,
    ) -> Result<()> {
        //TODO check if song is already proposed
        Self::collection::<Party>(self, "parties")
            .update_one(
                doc! {
                    "_id": partyid
                },
                doc! {
                    "$push": {"queue": {"votes": [{
                        "voter": session,
                        "opinion": 1
                    }],
                    "rank": 1,
                    "songid": songid}}
                },
                None,
            )
            .await?;
        Ok(())
    }

    pub async fn pop_popular_song(&self, partyid: String, password: String) -> Result<ObjectId> {
        let parties = Self::collection::<Document>(self, "parties");

        // set currentSong to the first element of the ordered queue
        parties
            .update_one(
                doc! {
                    "_id": &partyid,
                    "password": &password
                },
                vec![doc! {"$set": {
                    "currentSong": {
                    "$first": "$queue.songid"
                }
                }}],
                None,
            )
            .await?;

        // remove the first element of queue
        parties
            .update_one(
                doc! {
                    "_id": &partyid,
                    "password": password
                },
                doc! {"$pop": {
                "queue": -1i32
                }},
                None,
            )
            .await?;

        let err = || anyhow!("no current song present");
        //return the currentSong
        let song = parties
            .aggregate(
                vec![
                    doc! {"$match": {
                    "_id": partyid,
                    }},
                    doc! {"$project": {"currentSong": 1i32}},
                ],
                None,
            )
            .await?
            .try_collect::<Vec<Document>>()
            .await?
            .get(0)
            .ok_or_else(&err)?
            .get("currentSong")
            .ok_or_else(&err)?
            .as_object_id()
            .ok_or_else(&err)?;
        Ok(song)
    }

    pub async fn toggle_live(&self, partyid: String, password: String) -> Result<()> {
        Self::collection::<Party>(self, "parties")
            .update_one(
                doc! {
                    "_id": partyid,
                    "password": password
                },
                vec![doc! {
                    "$set": {
                        "isLive": {
                         "$eq": [false, "$isLive"]
                        }
                    }
                }],
                None,
            )
            .await?;
        Ok(())
    }
}
