mod api;

use api::{
    show_queue,
    get_session,
    vote,
    propose,
    create_party,
    next,
    toggle
};
use actix_web::{
    web,
    App,
    HttpServer,
};
use actix_cors::Cors;
use actix_web::middleware::Logger;
use mongodb::Client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let uri = std::env::var("MONGODB_URI")
        .unwrap();
    let client = Client::with_uri_str(uri)
        .await
        .unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_method()
            .allow_any_origin();
        let logger = Logger::default();
        App::new()
            .app_data(web::Data::new(client.clone()))
            .wrap(cors)
            .wrap(logger)
            .service(show_queue)
            .service(get_session)
            .service(vote)
            .service(propose)
            .service(create_party)
            .service(next)
            .service(toggle)
    })
    .bind(("0.0.0.0", 1337))?
    .run()
    .await
}
