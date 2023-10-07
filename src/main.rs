// modules
mod db;
mod models;
mod server;

use std::fmt;
use std::fmt::Formatter;

// lib usages
use dotenv::dotenv;
use mongodb::{error::Result, Client};

use crate::db::connect_to_mongodb;
use crate::server::start_app;

pub struct GlobalInfo {
    db_connection: Result<Client>,
}
impl fmt::Debug for GlobalInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        return f.debug_struct("Global Info")
            .field("Connection", &self.db_connection)
            .finish()
    }
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
