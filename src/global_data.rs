use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::data_base::DBConfig;

pub static GLOBAL_DATA: Lazy<Mutex<GlobalData>> = Lazy::new(|| {
    Mutex::new(GlobalData::new())
});

#[derive(Debug)]
pub struct GlobalData {
    db_config: Option<DBConfig>,
}

impl GlobalData {
    pub fn new() -> GlobalData {
        GlobalData {
            db_config: None
        }
    }
    pub fn get_db_config(&self) -> Option<&DBConfig> {
        self.db_config.as_ref()
    }
    pub fn set_db_config(&mut self, config: DBConfig) {
        self.db_config = Some(config);
    }
}
