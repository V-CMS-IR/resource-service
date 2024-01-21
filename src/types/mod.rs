use async_graphql::scalar;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize , Deserialize , Debug)]
pub struct DateWrapper<T>(T);

impl Default for DateWrapper<DateTime>{
    fn default() -> Self {
        DateWrapper(DateTime::now())
    }
}
scalar!(DateWrapper<DateTime>);