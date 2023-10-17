use mongodb::{error::Result, options::ClientOptions, Client};
use std::env::var;
use std::time::Duration;
use mongodb::{Database};


#[derive(Debug, Clone)]
pub struct DBConfig {
    pub client: Client,
    pub valid_databases: Vec<String>
}
#[derive(Debug)]
pub struct DB(pub Database);

pub(super) async fn connect_to_mongodb() -> Result<DBConfig> {
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
    let db = initial_data_bases(client).await?;
    Ok(db)
}

async fn check_db_connection(client: &Client) -> Vec<String> {
    client.list_database_names(None, None).await.unwrap()
}

async fn initial_data_bases(client: Client) -> Result<DBConfig> {
    let data_bases = check_db_connection(&client).await;
    let db = DBConfig {
        client,
        valid_databases: data_bases
    };
    Ok(db)
}
