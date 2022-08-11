use config::Mongo;
use async_graphql::Schema;
use crate::handler::endpoints::services;
use handler::{Mutation, Query, Subscription};
use actix_web::{
    middleware::Logger, web::Data, App, HttpServer
};

mod config;
mod handler;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let db = Mongo::init().await;
    let schema = Schema::build(Query, Mutation, Subscription)
        .data(db)
        .finish();

    HttpServer::new(move || {
        App::new()
            .configure(services)
            .wrap(Logger::default())
            .app_data(Data::new(schema.clone()))
    })
    .workers(2)
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
