// modules
mod db;
mod models;
mod server;

// lib usages
use dotenv::dotenv;
use mongodb::{error::Result, Client};

use crate::db::connect_to_mongodb;
use crate::server::start_app;

#[derive(Debug)]
pub struct GlobalInfo {
    db_connection: Result<Client>,
}

#[tokio::main]
async fn main() {
    #![allow(warnings)]
    load_env();
    let info = GlobalInfo {
        db_connection: connect_to_mongodb().await,
    };
    start_app(info).await;
}

fn load_env() {
    dotenv().ok();
}
