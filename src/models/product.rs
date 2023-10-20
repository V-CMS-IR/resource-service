use async_graphql::{Object, Result, SimpleObject, Enum, Context , Error};
use mongodb::{Database};
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

#[derive(Default)]
pub struct ProductQuery;

#[derive(Default)]
pub struct ProductMutation;

impl Product {
    pub fn new(title: String, creator: Creator, description: Option<String> , status: Option<Status>) -> Self {
        Self {
            title,
            description,
            status: match status { Some(status) => status , None => Status::Scheduled },
            id: None,
            content: None,
            meta: None,
            creator,
        }
    }
}

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
    ) -> Result<String , Error> {
        let product = Product::new(title, 0, description , status);
        let db = ctx.data::<Database>().unwrap();
        let collection = db.collection::<Product>("Products");
        let insert = collection.insert_one(&product, None).await?;
        match insert.inserted_id.as_object_id() {
            Some(id) => Ok(id.to_string()),
            None => Err(Error::new("Nothing inserted"))
        }
    }
}
