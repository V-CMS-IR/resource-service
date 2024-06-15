use std::ops::{Deref, DerefMut};
use async_graphql::scalar;
use chrono::{Local, TimeZone};
use serde::Deserialize;
use spark_orm::Serialize;

pub type Error = async_graphql::Error;

pub type Result<T> = std::result::Result<T , Error>;


#[derive(Serialize , Deserialize , Debug)]
pub struct DateTime(mongodb::bson::DateTime);
scalar!(DateTime);
impl Default for DateTime {
    fn default() -> Self {
        DateTime{
            0: mongodb::bson::DateTime::now()
        }
    }
}
impl From<String> for DateTime{
    fn from(value: String) -> Self {
        let native_date = chrono::NaiveDateTime::parse_from_str(&value , "%Y-%m-%d %H:%M:%S").unwrap();
        let date_time: chrono::DateTime<Local> = Local.from_local_datetime(&native_date).unwrap();
        DateTime(mongodb::bson::DateTime::from_chrono(
            date_time
        ))
    }
}

impl Deref for DateTime {
    type Target = mongodb::bson::DateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DateTime{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

