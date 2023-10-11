use crate::models::{MainMutation, MainQuery};
use crate::DB;
use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use axum::Router;
use axum::routing::post_service;



pub async fn start_app(db: DB) {
    let schema = Schema::build(MainQuery::default(), MainMutation, EmptySubscription)
        .data(db)
        .finish();

    let app = Router::new().route("/", post_service(GraphQL::new(schema)));

    println!("GraphiQL IDE: http://localhost:8000");

    axum::Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
