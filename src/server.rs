pub mod middleware;
mod gp_middleware;

use crate::models::{MainMutation, MainQuery};
use crate::data_base::DBConfig;
use async_graphql::{EmptySubscription, Schema};
use axum::{Router, routing::{post}, Server};
use crate::server::gp_middleware::GraphQlLifeCycle;
use crate::server::middleware::specify_db;

#[derive(Clone)]
pub struct AppState {
    schema: Schema<MainQuery, MainMutation, EmptySubscription>,
    db: DBConfig,
}

pub async fn start_app(db: DBConfig) {
    let schema = Schema::build(MainQuery::default(), MainMutation::default(), EmptySubscription)
        .extension(GraphQlLifeCycle)
        .finish();

    let app = Router::new()
        .route("/", post(specify_db))
        // .route("/ws", get(graphql_ws_handler))
        .with_state(AppState {
            schema,
            db,
        });

    println!("Playground: http://localhost:8000");

    Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
