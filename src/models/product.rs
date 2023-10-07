use async_graphql::{Json, Object, Result, SimpleObject};
use serde::{Deserialize, Serialize};
use crate::models::MainMutation;

#[derive(SimpleObject , Debug , Serialize ,  Deserialize)]
pub struct Product{
    #[graphql(skip)]
    id: Option<String>,
    // mongodb id
    pub title: String,
    // The post title
    pub description: Option<String>,
    pub content: Option<String>,
}

impl Product {
    pub fn new(title: String, description: Option<String>) -> Self {
        Self {
            title,
            description,
            id: None,
            content: None,
        }
    }
}

#[Object]
impl MainMutation {
    async fn new_product(&self , data: Json<Product>) -> Result<Product> {
        Ok(Product::new(data.title.clone() , data.description.clone()))
    }
}