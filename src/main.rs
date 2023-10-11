// modules
mod data_base;
mod models;
mod server;

use std::env::var;
// lib usages
use dotenv::dotenv;
use mongodb::{Collection, Database };
use mongodb::bson::Document;



use crate::data_base::connect_to_mongodb;
use crate::server::start_app;

#[derive(Debug)]
pub struct DB {
    global: Database,
    sites_map: Collection<Document>,
    cms: Database,
}

#[tokio::main]
async fn main() {
    #![allow(warnings)]
    load_env();

    // TODO move this configs to data_base.rs file
    let client = connect_to_mongodb().await.unwrap();
    let global_name = var("DB_GLOBAL_NAME").expect("the DB_SITES_MAP_NAME is not set");
    let cms_name = var("DB_CMS_NAME").expect("the DB_CMS_NAME is not set");
    let sites_map = client.database(&global_name).collection::<Document>("sites_map");
    let db = DB{
        global : client.database(&global_name),
        cms: client.database(&cms_name),
        sites_map,
    };

    start_app(db).await;
}

fn load_env() {
    dotenv().ok();
}
