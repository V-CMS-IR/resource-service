use crate::models::{MainMutation, MainQuery};
use crate::GlobalInfo;
use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use axum::Router;
use axum::routing::post_service;



pub async fn start_app(info: GlobalInfo) {
    let schema = Schema::build(MainQuery, MainMutation, EmptySubscription)
        .data(info)
        .finish();

    let app = Router::new().route("/", post_service(GraphQL::new(schema)));

    println!("GraphiQL IDE: http://localhost:8000");

    axum::Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
