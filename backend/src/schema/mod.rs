use async_graphql::{
    InputObject, InputValueError, InputValueResult, Scalar, ScalarType, SimpleObject, Value,
};
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
    pub address: String,
}

#[derive(serde::Deserialize, Clone, Copy, Debug)]
pub enum Opinion {
    Like = 1,
    Neutral = 0,
    Dislike = -1,
}

#[Scalar]
impl ScalarType for Opinion {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::Number(n) = &value {
            if let Some(n) = n.as_i64() {
                return match n {
                    1 => Ok(Opinion::Like),
                    0 => Ok(Opinion::Neutral),
                    -1 => Ok(Opinion::Dislike),
                    _ => Err(InputValueError::expected_type(value)),
                };
            }
        }
        Err(InputValueError::expected_type(value))
    }

    fn to_value(&self) -> Value {
        Value::Number((*self as i32).into())
    }
}

impl Opinion {
    pub fn int(self) -> i64 {
        self as i64
    }
}
