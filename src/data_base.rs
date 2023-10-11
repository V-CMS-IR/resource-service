use mongodb::{error::Result, options::ClientOptions, Client};
use  std::env::var;
use std::time::Duration;

pub async fn connect_to_mongodb() -> Result<Client> {
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
    Ok(client)
}

async fn check_db_connection(client: &Client){
    let database_names = client.list_database_names(None, None).await.unwrap();
    println!("Connected to MongoDB. Database names: {:?}", database_names);

}

