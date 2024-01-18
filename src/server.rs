mod graphql_lifecycle;
pub mod middleware;

use crate::models::{MainMutation, MainQuery};
use crate::server::graphql_lifecycle::GraphQlLifeCycle;
use crate::server::middleware::specify_db;
use async_graphql::{EmptySubscription, Schema};
use axum::{routing::post, Router};

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
        .route("/", post(specify_db))
        // .route("/ws", get(graphql_ws_handler))
        .with_state(AppState { schema });
    let addr = "127.0.0.1:8000";
    let bind = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|_| panic!("Can't bind the address {} ", addr));
    let _ = axum::serve(bind, app.into_make_service()).await;
}
