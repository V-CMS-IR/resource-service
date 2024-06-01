use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use spark_orm::Model;
use crate::app::models::category::Category;
use crate::types::DateWrapper;

#[Model(coll_name = "brands")]
#[derive(SimpleObject, Serialize, Deserialize, Default, Debug)]
pub struct Brand {
    pub title: String,
    pub categories: Vec<Category>,
    pub created_at: DateWrapper,
    pub updated_at: DateWrapper,
    pub deleted_at: DateWrapper,
}