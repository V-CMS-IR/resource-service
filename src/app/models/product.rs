use async_graphql::{Enum, Error, Object, Result, SimpleObject};
use mongodb::bson::{DateTime, doc};
use mongodb::Database;
use rm_orm::preload::*;
use serde::{Deserialize, Serialize};

use crate::types::DateWrapper;

//TODO Write A Comment For all section of it and separate Sections
type Creator = i16;


#[derive(Model, Default, SimpleObject, Debug, Serialize, Deserialize)]
#[coll_name = "Products"]
pub struct Product {
    #[graphql(skip)]
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub content: Option<Content>,
    pub status: Status,
    pub meta: Option<Vec<Meta>>,
    pub author: Creator,
    pub created_at:  DateWrapper<DateTime>,
    pub update_at: DateWrapper<DateTime>,
    pub deleted_at: DateWrapper<DateTime>
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
        title: String,
        description: Option<String>,
        status: Option<Status>,
    ) -> Result<String, Error> {
        let db = RmORM::get_db();
        let mut product = Product::new(&db).await;
        product.title = title;
        product.description = description;
        product.status = status.unwrap_or_default();
        let re = product.save().await?;
        Ok(re.inserted_id.to_string())
    }
}


