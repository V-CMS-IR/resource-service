// modules
mod data_base;
mod server;
mod types;
mod app;

// lib usages
use dotenv::dotenv;

use crate::data_base::init_db;
use crate::server::start_app;

fn load_env() {
    dotenv().ok();
}

#[tokio::main]
async fn main() {
    #![allow(warnings)]
    load_env();
    init_db().await;
    start_app().await;
}
