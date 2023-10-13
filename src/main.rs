// modules
mod data_base;
mod models;
mod server;

// lib usages
use dotenv::dotenv;
use crate::data_base::connect_to_mongodb;
use crate::server::start_app;


#[tokio::main]
async fn main() {
    #![allow(warnings)]
    load_env();
    // TODO move this configs to data_base.rs file
    let db= connect_to_mongodb().await.unwrap();
    start_app(db).await;
}

fn load_env() {
    dotenv().ok();
}
