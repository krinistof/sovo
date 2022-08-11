use async_graphql::{InputObject, SimpleObject};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Song {
    pub _id: ObjectId,
    pub artist: String,
    pub title: String,
    pub duration: String,
}

#[derive(Debug, InputObject)]
pub struct FetchSong {
    pub _id: String,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
pub struct Party {
    _id: String,
    queue: Option<Vec<Song>>,
    is_live: bool,
    current_song: Option<ObjectId>,
    password: String,
}

#[derive(Debug, InputObject)]
pub struct CreateParty {
    partyid: String,
    password: String,
}

#[derive(Debug, InputObject)]
pub struct Vote {
    partyid: String,
    songid: String,
    sessionid: String,
    is_like: bool,
}

#[derive(Debug, InputObject)]
pub struct Propose {
    partyid: String,
    songid: String,
    sessionid: String,
}

#[derive(Debug, InputObject)]
pub struct FetchQueue {
    partyid: String,
}

#[derive(Debug, InputObject)]
pub struct FetchSession {
    _id: String,
    joined: String,
}

#[derive(serde::Deserialize)]
pub struct Session {
    pub _id: ObjectId,
    pub address: String  
}