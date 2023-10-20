use mongodb::{error::Result, options::ClientOptions, Client};
use std::env::var;
use std::time::Duration;
use crate::GLOBAL_DATA;

#[derive(Debug)]
pub struct DBConfig {
    pub client: Client,
    pub valid_database: Vec<String>,
}

pub async fn init_db() {
    let mut global_data = GLOBAL_DATA.lock().unwrap();
    let client = get_mongo_client().await.unwrap();
    check_db_connection(&client).await;
    let db_config = DBConfig {
        client,
        valid_database: vec!["localhost:8000".to_string()],
    };
    global_data.set_db_config(db_config);
    drop(global_data);
}

pub async fn get_mongo_client() -> Result<Client> {
    let db_user = var("CMS_DB_USER").expect("the DB_USER in not set");
    let db_pass = var("CMS_DB_PASS").expect("the DB_PASS in not set");
    let db_host = var("CMS_DB_HOST").expect("the DB_HOST is not set");
    let db_port = var("CMS_DB_PORT").expect("the DB_PORT in not set");
    let connection_string = format!(
        "mongodb://{}:{}@{}:{}",
        db_user, db_pass, db_host, db_port
    );
    let mut client_options = ClientOptions::parse(connection_string).await.unwrap();
    client_options.connect_timeout = Some(Duration::from_secs(1));
    let client = Client::with_options(client_options).unwrap();
    Ok(client)
}

async fn check_db_connection(client: &Client) {
    client.list_database_names(None, None).await.unwrap();
}

impl DBConfig {
    pub fn get_client(&self) -> &Client {
        &self.client
    }
    pub fn get_data_bases(&self) -> &Vec<String> {
        &self.valid_database
    }
}
