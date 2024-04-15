#![allow(clippy::await_holding_lock)]

use spark_orm::Spark;
use std::env::var;
#[derive(Debug)]
pub struct DBConfig {
    pub valid_database: Vec<String>,
}

pub async fn init_db() {
    let db_user = var("CMS_DB_USER").expect("the DB_USER in not set");
    let db_pass = var("CMS_DB_PASS").expect("the DB_PASS in not set");
    let db_host = var("CMS_DB_HOST").expect("the DB_HOST is not set");
    let db_port = var("CMS_DB_PORT").expect("the DB_PORT in not set");
    Spark::global_connect(&db_user, &db_pass, &db_host, &db_port, "localhost:8000").await;
}
