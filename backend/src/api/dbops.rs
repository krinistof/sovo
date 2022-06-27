use actix_web::web;
use mongodb::{Client, Collection, Cursor, error::Result, results::{InsertOneResult, UpdateResult}};
use bson::{doc, Document, oid::ObjectId, DateTime, Bson::Null};
use mongodb::options::UpdateOptions;
use crate::api::Party;

pub async fn create_session(database: web::Data<Client>)
    -> Result<InsertOneResult> {
    database
        .database("sovo")
        .collection::<Document>("sessions")
        .insert_one(doc! {
            "joined": DateTime::now()
        }, None)
        .await
}

pub async fn get_queue(database: web::Data<Client>, session: ObjectId, partyid: &String)
    -> Result<Cursor<Document>> {

    database
        .database("sovo")
        .collection::<Party>("parties")
        .aggregate(
            vec![
                doc!{"$match": {"_id": partyid}},
                doc!{
                    "$project": {
                        "currentSong": 1,
                        "isLive": 1,
                        "queue": {
                            "$map": {
                                "input": "$queue",
                                "in": {
                                    "likeStatus": {"$let": {
                                        "vars": {
                                            "session": { "$last": {"$filter": {
                                                "input": "$$this.votes",
                                                "as": "vote",
                                                "cond": {
                                                    "$eq":
                                                    ["$$vote.voter",session]
                                                }
                                        }}}},
                                        "in": { "$cond": {
                                            "if": "$$session",
                                            "then": "$$session.opinion",
                                            "else": "0"
                                        }}
                                    }
                                    },
                                    "songid": "$$this.songid",
                                    "rank": "$$this.rank",
                                }
                            }
                        }
                    }
                }
            ],
             None
        )
        .await
}

pub async fn process_vote(database: web::Data<Client>,
                           session: ObjectId,
                           partyid: &String,
                          song_oid: ObjectId,
                           is_like: bool) -> Result<UpdateResult>{
    // remove previous vote if any

    let parties = database
        .database("sovo")
        .collection::<Party>("parties");

    parties
        .update_one(doc! {
            "_id": partyid
        }, doc! {
            "$pull": { "queue.$[element].votes": {
                    "voter": session
                }
            }
        }, UpdateOptions::builder().array_filters(
            vec![doc! {"element.songid": song_oid}]
        ).build())
        .await
        .expect("Database error");

    // update song with vote
    let opinion: i32 = if is_like { 1 } else {-1};
    parties
        .update_one(doc!{
            "_id": partyid
        }, doc! {
            "$push": {"queue.$[element].votes" : {
                "voter": session,
                "opinion": opinion
            }}
        }, UpdateOptions::builder().array_filters(
            vec![ doc!{"element.songid": song_oid} ]
        ).build())
        .await
        .expect("Database error");

    // order queue
    sort_by_rank(&parties, partyid).await
}

pub async fn add_propose(database: web::Data<Client>,
                         session: ObjectId,
                         partyid: &String,
                         songid: ObjectId) -> Result<UpdateResult> {
    database.database("sovo")
        .collection::<Party>("parties")
        .update_one(
            doc! {
                "_id": partyid
            },
            doc!{
                // TODO push may be more effective
                "$addToSet": {"queue": {"votes": [{
                    "voter": session,
                    "opinion": 1
                }],
                "rank": 1,
                "songid": songid}}
            },
            None
        )
        .await
}

async fn calculate_rank(collection: &Collection<Party>, partyid: &String)
    -> Result<UpdateResult> {
    collection
        .update_one(
            doc! {
                "_id": partyid
            },
            vec![doc!{
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
            None
        )
        .await
}

pub async fn sort_by_rank(collection: &Collection<Party>, partyid: &String)
    -> Result<UpdateResult> {
    calculate_rank(collection, partyid)
        .await
        .expect("Database error");
    collection
        .update_one(
            doc!{ "_id": partyid },
            doc!{
                "$push": {
                    "queue": {
                        "$each": [],
                        "$sort": {"rank": -1}
                    }
                }
            },
            None
        )
        .await
}

pub async fn add_party(database: web::Data<Client>,
                    partyid: &String,
                   password: &String
        ) -> Result<InsertOneResult> {
    database.database("sovo")
        .collection::<Document>("parties")
        .insert_one(
            doc! {
                "_id": partyid,
                "isLive": false,
                "currentSong": Null,
                "password": password,
                "queue": [],
            },
            None
        )
        .await
}
pub async fn pop_popular_song(database: web::Data<Client>,
                               partyid: &String,
                              password: &String
    ) -> Result<Cursor<Document>> {
    let parties =
    database.database("sovo")
        .collection::<Document>("parties");

    // set currentSong to the first element of the ordered queue
    parties.update_one(
            doc!{
                "_id": partyid,
                "password": password
            },
            vec![doc!{"$set": {
                "currentSong": {
                    "$first": "$queue.songid"
                }
            }}],
            None
        )
        .await
        .expect("Database error");

    // remove the first element of queue
    parties.update_one(
            doc!{
                "_id": partyid,
                "password": password
            },
            doc!{"$pop": {
                "queue": -1i32
            }},
            None
        )
        .await
        .expect("Database error");

    //return the currentSong
    parties.aggregate(
            vec![
                doc!{"$match": {
                    "_id": partyid,
                    "password": password
                }},
                doc!{"$project": {"currentSong": 1}}
            ],
            None
        )
        .await
}

pub async fn toggle_live(database: web::Data<Client>,
                     partyid: &String,
                     password: &String
    ) -> Result<UpdateResult> {
    database.database("sovo")
        .collection::<Party>("parties")
        .update_one(
            doc!{
                "_id": partyid,
                "password": password
            },
            vec![doc!{
                "$set": {
                    "isLive": {
                        "$eq": [false, "$isLive"]
                    }
                }
            }],
            None
        )
        .await
}
