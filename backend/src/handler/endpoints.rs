use actix_web::{
    web, HttpRequest, HttpResponse, Result
};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Schema, Data
};
use async_graphql_actix_web::{
    GraphQLRequest, GraphQLResponse, GraphQLSubscription
};
use crate::handler::SovoSchema;

pub async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        )))
}

pub async fn index(
    schema: web::Data<SovoSchema>,
    req: HttpRequest,
    gql_req: GraphQLRequest
) -> GraphQLResponse {
    let mut request = gql_req.into_inner();
    if let Some(addr) = req.peer_addr() {
        request = request.data(addr.ip().to_string());
    }
    schema.execute(request).await.into()
}

pub async fn index_ws(
    schema: web::Data<SovoSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    let mut data = Data::default();
    if let Some(addr) = req.peer_addr() {
        data.insert(addr.ip().to_string());
    }

    GraphQLSubscription::new(Schema::clone(&*schema))
        .with_data(data)
        .start(&req, payload)
}
