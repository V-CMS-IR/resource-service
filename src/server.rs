pub mod middleware;
mod graphql_lifecycle;

use crate::models::{MainMutation, MainQuery};
use async_graphql::{EmptySubscription, Schema};
use axum::{Router, routing::{post}, Server};
use crate::server::graphql_lifecycle::GraphQlLifeCycle;
use crate::server::middleware::{specify_db};

#[derive(Clone)]
pub struct AppState {
    schema: Schema<MainQuery, MainMutation, EmptySubscription>,
}

pub async fn start_app() {
    let schema = Schema::build(MainQuery::default(), MainMutation::default(), EmptySubscription)
        .extension(GraphQlLifeCycle)
        .finish();
    let app = Router::new()
        .route("/", post(specify_db))
        // .route("/ws", get(graphql_ws_handler))
        .with_state(AppState {
            schema,
        });

    println!("Playground: http://localhost:8000");

    Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
