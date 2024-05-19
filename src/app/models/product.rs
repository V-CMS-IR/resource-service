use async_graphql::{Enum, SimpleObject};
use spark_orm::{Model};
use serde::{Deserialize, Serialize};
use crate::types::{DateWrapper, ObjectID};

//TODO Write A Comment For all section of it and separate Sections
pub type Creator = i16;
pub type Price = f32;

#[Model(coll_name = "Products")]
#[derive(SimpleObject, Serialize, Deserialize, Default, Debug)]
pub struct Product {
    #[serde(skip_serializing_if = "ObjectID::is_none")]
    pub _id: ObjectID,
    pub category_id: ObjectID,
    pub title: String,
    pub description: Option<String>,
    pub content: Option<Content>,
    pub status: Status,
    pub meta: Option<Vec<Meta>>,
    pub author: Creator,
    pub price: Price,
    pub created_at: DateWrapper,
    pub updated_at: DateWrapper,
    pub deleted_at: DateWrapper,
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

