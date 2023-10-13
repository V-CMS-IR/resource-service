use mongodb::{error::Result, options::ClientOptions, Client};
use std::env::var;
use std::time::Duration;
use mongodb::{Database};


#[derive(Debug, Clone)]
pub struct DBConfig {
    pub global: Database,
    pub client: Client,
}

pub struct DB(pub Database);

pub(super) async fn connect_to_mongodb() -> Result<DBConfig> {
    let db_user = var("DB_USER").expect("the DB_USER in not set");
    let db_pass = var("DB_PASS").expect("the DB_PASS in not set");
    let db_host = var("DB_HOST").expect("the DB_HOST is not set");
    let db_port = var("DB_PORT").expect("the DB_PORT in not set");
    let connection_string = format!(
        "mongodb://{}:{}@{}:{}",
        db_user, db_pass, db_host, db_port
    );
    let mut client_options = ClientOptions::parse(connection_string).await.unwrap();
    client_options.connect_timeout = Some(Duration::from_secs(1));
    let client = Client::with_options(client_options).unwrap();
    check_db_connection(&client).await;
    let db = initial_data_bases(client).await?;
    Ok(db)
}

async fn check_db_connection(client: &Client) {
    client.list_database_names(None, None).await.unwrap();
}

async fn initial_data_bases(client: Client) -> Result<DBConfig> {
    let global_name = var("DB_GLOBAL_NAME").expect("the DB_SITES_MAP_NAME is not set");
    let db = DBConfig {
        global: client.database(&global_name),
        client,
    };
    Ok(db)
}
