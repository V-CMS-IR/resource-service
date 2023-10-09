use async_graphql::{Object, Result, SimpleObject, Enum};
use serde::{Deserialize, Serialize};

//TODO Write A Comment For all section of it and separate Sections
type Creator = i16;

#[derive(SimpleObject, Debug, Serialize, Deserialize)]
pub struct Product {
    #[graphql(skip)]
    id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub content: Option<Content>,
    pub status: Status,
    pub meta: Option<Vec<Meta>>,
    pub creator: Creator,
}

#[derive(SimpleObject, Debug, Serialize, Deserialize)]
pub struct Meta {
    name: String,
    value: String,
}

#[derive(SimpleObject, Debug, Serialize, Deserialize)]
pub struct Content {
    pub params: Vec<Param>,
    pub raw_content: String,
}

#[derive(SimpleObject, Debug, Serialize, Deserialize)]
pub struct Param {
    pub name: String,
    pub component: String,
    pub value: String,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq , Debug ,Serialize , Deserialize)]
pub enum Status {
    Published,
    Draft,
    Scheduled,
    Pending,
}

impl Product {
    pub fn new(title: String, creator: Creator, description: Option<String>) -> Self {
        Self {
            title,
            description,
            status: Status::Scheduled,
            id: None,
            content: None,
            meta: None,
            creator,
        }
    }
}

#[derive(Default)]
pub struct ProductQuery;
#[derive(Default)]
pub struct ProductMutation;
#[Object]
impl ProductQuery {
    async fn get_enums(&self , name: Status) -> Result<Status>{
        Ok(name)
    }
}

#[Object]
impl ProductMutation {
    async fn new_product(
        &self,
        title: String,
        description: Option<String>,
        status: Option<Status>
    ) -> Result<Product> {
        Ok(Product::new("".to_string(), 0, None ))
    }
}
