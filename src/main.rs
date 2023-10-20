// modules
mod data_base;
mod models;
mod server;

use std::sync::Mutex;
// lib usages
use dotenv::dotenv;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct GlobalData {
    db_config: Option<DBConfig>
}
impl GlobalData{
    pub fn new()-> GlobalData{
        GlobalData {
            db_config: None
        }
    }
    pub fn get_db_config(&self) -> Option<&DBConfig> {
        self.db_config.as_ref()
    }
    pub fn set_db_config(&mut self , config: DBConfig){
        self.db_config = Some(config);
    }
}

pub static GLOBAL_DATA: Lazy<Mutex<GlobalData>> = Lazy::new(||{
    Mutex::new(GlobalData::new())
});


use crate::data_base::{DBConfig, init_db};
use crate::server::start_app;

#[tokio::main]
async fn main() {
    #![allow(warnings)]
    load_env();
    init_db().await;
    start_app().await;
}

fn load_env() {
    dotenv().ok();
}
