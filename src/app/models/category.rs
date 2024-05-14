use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use spark_orm::Model;
use crate::app::models::product::Product;
use crate::types::DateWrapper;

#[Model(coll_name="categories")]
#[derive(SimpleObject , Serialize , Deserialize , Default ,  Debug)]
pub struct Category {
    // #[unique]
    pub slug: String,
    pub products: Vec<Product>,
    pub title: String,
    pub created_at: DateWrapper,
    pub updated_at: DateWrapper,
    pub deleted_at: DateWrapper,
}