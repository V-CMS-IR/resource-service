use async_graphql::{Object, Result, SimpleObject, Enum, Context};
use serde::{Deserialize, Serialize};
use crate::data_base::{DB};

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

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Status {
    Published,
    Draft,
    Scheduled,
    Pending,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum MutationStatus {
    Success,
    Failed,
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
    async fn get_enums(&self, name: Status) -> Result<Status> {
        Ok(name)
    }
}

#[Object]
impl ProductMutation {
    async fn new_product<'a>(
        &self,
        ctx: &Context<'_>,
        title: String,
        description: Option<String>,
        status: Option<Status>,
    ) -> Result<MutationStatus> {
        let product = Product::new(title, 0, description);
        let db = ctx.data::<DB>()?;
        let collection = db.0.collection::<Product>("Products");
        collection.insert_one(&product , None).await.unwrap();
        Ok(MutationStatus::Success)
    }
}
