use mongodb::{
    options::ClientOptions,
    Client,
    error::Result,
};
use std::env::var;
use dotenv::dotenv;


#[tokio::main]
async fn main() {
    #![allow(warnings)]
    load_env();
    let client = connect_to_mongodb().await;
}

async fn connect_to_mongodb() -> Result<Client> {
    let db_user = var("DB_USER").expect("the DB_USER in not set");
    let db_pass = var("DB_PASS").expect("the DB_PASS in not set");
    let db_host = var("DB_HOST").expect("the DB_HOST is not set");
    let db_port = var("DB_PORT").expect("the DB_PORT in not set");
    let db_name = var("DB_NAME").expect("the DB_NAME is not set");
    let connection_string = format!(
        "mongodb://{}:{}@{}:{}/{}",
        db_user , db_pass , db_host , db_port , db_name
    );
    let client_options = ClientOptions::parse(connection_string).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    return Ok(client);
}

fn load_env() {
    dotenv().ok();
}
