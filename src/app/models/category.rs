use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use spark_orm::Model;
use crate::types::DateWrapper;

#[Model(coll_name="categories")]
#[derive(SimpleObject , Serialize , Deserialize , Default ,  Debug)]
#[graphql(complex)]
pub struct Category {
    // #[unique]
    pub slug: String,
    pub title: String,
    pub created_at: DateWrapper,
    pub updated_at: DateWrapper,
    pub deleted_at: DateWrapper,
}
