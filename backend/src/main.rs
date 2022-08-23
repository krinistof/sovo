use actix_web::{guard, middleware::Logger, web, web::Data, App, HttpServer};
use async_graphql::Schema;
use config::Mongo;
use handler::{
    endpoints::{index, index_playground, index_ws},
    Mutation, Query, Subscription,
};

mod config;
mod handler;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let db = Mongo::init().await;
    let schema = Schema::build(Query, Mutation, Subscription)
        .data(db)
        .finish();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(index_ws),
            )
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .workers(2)
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
