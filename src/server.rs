mod graphql_lifecycle;
pub mod middleware;

use std::env::var;
use crate::app::models::{MainMutation, MainQuery};
use crate::server::graphql_lifecycle::GraphQlLifeCycle;
use crate::server::middleware::execute_gql;
use async_graphql::{EmptySubscription, Schema};
use axum::{routing::post, Router};
use axum::extract::DefaultBodyLimit;

#[derive(Clone)]
pub struct AppState {
    schema: Schema<MainQuery, MainMutation, EmptySubscription>,
}

pub async fn start_app() {
    let schema = Schema::build(
        MainQuery::default(),
        MainMutation::default(),
        EmptySubscription,
    )
        .extension(GraphQlLifeCycle)
        .finish();
    let app = Router::new()
        .route("/", post(execute_gql))
        // .layer(
        //     DefaultBodyLimit::disable()
        // )
        // .route("/ws", get(graphql_ws_handler))
        .with_state(AppState { schema });

    let host = var("APP_HOST").expect("the APP_HOST in not set");
    let port = var("APP_PORT").expect("the APP_PORT in not set");

    let addr = &format!("{host}:{port}");
    let bind = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|_| panic!("Can't bind the address {} ", addr));
    println!("Server Start at {} ", addr);
    let _ = axum::serve(bind, app.into_make_service()).await;
}
