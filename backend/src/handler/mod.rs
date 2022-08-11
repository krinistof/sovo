use async_graphql::{Context, Object, Schema, Subscription};
use mongodb::bson::oid::ObjectId;
use futures_util::Stream;
use anyhow::Result;
use crate::config::Mongo;

pub mod endpoints;

//TODO dev thing, remove
use std::time::Duration;

pub type SovoSchema = Schema<Query, Mutation, Subscription>;

pub struct Query;

#[Object()]
impl Query {
    async fn test(&self, _ctx: &Context<'_>) -> String {
        "It works!!".to_owned()
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_session(&self, ctx: &Context<'_>) -> Result<ObjectId> {
        let db = ctx.data_unchecked::<Mongo>();
        let addr = ctx.data_unchecked::<String>();
        db.create_session(addr.to_string()).await
    }

    async fn test(&self, _ctx: &Context<'_>) -> String {
        "xdlol".to_owned()
    }
}

pub struct Subscription;

#[Subscription]
impl Subscription {
    async fn interval(&self, #[graphql(default = 1)] n: i32) -> impl Stream<Item = i32> {
        let mut value = 0;
        async_stream::stream! {
            loop {
                futures_timer::Delay::new(Duration::from_secs(1)).await;
                value += n;
                yield value;
            }
        }
    }
}
