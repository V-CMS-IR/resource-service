use async_graphql::{Enum, InputObject, SimpleObject};
use mongodb::bson::oid::ObjectId;
use spark_orm::{Model};
use serde::{Deserialize, Serialize};

//TODO Write A Comment For all section of it and separate Sections
pub type Price = f32;

#[Model(coll_name = "products")]
#[derive(SimpleObject, Serialize, Deserialize, Default, Debug)]
pub struct Product {
    pub title: String,
    pub description: Option<String>,
    pub content: Option<Content>,
    pub status: Status,
    pub meta: Vec<Meta>,
    pub price: Price,
    pub brand_id: ObjectId,
}

#[derive(InputObject , Serialize , Deserialize , Default , Debug)]
pub struct ProductInput{
    pub title: String,
    pub description: Option<String>,
    pub content: Option<Content>,
    pub status: Status,
    pub meta: Vec<Meta>,
    pub price: Price,
    pub brand_id: ObjectId,
}

#[derive(SimpleObject, Debug, Serialize, Deserialize , InputObject)]
#[graphql(input_name="MetaInput")]
pub struct Meta {
    name: String,
    value: String,
}

#[derive(SimpleObject, Debug, Serialize, Deserialize , InputObject)]
#[graphql(input_name="ContentInput")]
pub struct Content {
    pub params: Vec<Param>,
    pub raw_content: String,
}

#[derive(SimpleObject, Debug, Serialize, Deserialize , InputObject)]
#[graphql(input_name="InputParam")]
pub struct Param {
    pub name: String,
    pub component: String,
    pub value: String,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize, Default)]
pub enum Status {
    Published,
    #[default]
    Draft,
    Scheduled,
    Pending,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum MutationStatus {
    Success,
    Failed,
}

