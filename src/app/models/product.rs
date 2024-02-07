use std::sync::{Arc, Mutex};
use async_graphql::{Enum, Error, Object, Result, SimpleObject};
use mongodb::bson::doc;
use rm_orm::{Model, ProxyModelCrud, RmORM};
use rm_orm::model::Prototype;
use serde::{Deserialize, Serialize};

use crate::types::{DateWrapper, ObjectID};

//TODO Write A Comment For all section of it and separate Sections
type Creator = i16;


// #[derive(Model, Default, SimpleObject, Debug, Serialize, Deserialize)]
// #[coll_name = "Products"]
#[Model(coll_name = "Products")]
#[derive(SimpleObject, Serialize, Deserialize, Default)]
pub struct Product {
    #[serde(skip_serializing_if = "ObjectID::is_none")]
    _id: ObjectID,
    pub title: String,
    pub description: Option<String>,
    pub content: Option<Content>,
    pub status: Status,
    pub meta: Option<Vec<Meta>>,
    pub author: Creator,
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

#[derive(Default)]
pub struct ProductQuery;

#[derive(Default)]
pub struct ProductMutation;

#[Object]
impl ProductQuery {
    async fn get_product(&self, object_id: Option<ObjectID>) -> Result<Option<Product>, Error> {
        let db = RmORM::get_db();
        let mut product = Product::new_model(&db);
        let mut sample = doc! {
            "author": 0
        };
        if object_id.is_some() {
            let ob = object_id.unwrap().0;
            sample.insert("_id", ob);
        }
        let re = product.find_one(
            Prototype::Doc(sample)
        ).await?;

        Ok(re)
    }
    async fn get_products(&self) -> Result<Vec<Product>, Error> {
        let db = RmORM::get_db();
        let product = Product::new_model(&db);
        let sample = Prototype::Doc(
            doc! {
                "author": 0
            }
        );
        let result = Arc::new(Mutex::new(Vec::new())); // Wrap result in an Arc and a Mutex
         product.find_with_callback(sample, {
            let result = Arc::clone(&result); // Clone Arc for the closure
            move |pr| {
                // Accessing result inside the closure
                let mut result = result.lock().unwrap(); // Obtain the lock
                result.push(pr); // Pushing a String into the vector
            }
        }).await;
        let arc_mutex_inner = Arc::try_unwrap(result).map_err(|_| Error::new("Can't get the result"))?;
        let mutex_inner = arc_mutex_inner.into_inner().map_err(|_| Error::new("Can't get the result"))?;
        Ok(mutex_inner)
    }
}

#[Object]
impl ProductMutation {
    async fn new_product<'a>(
        &self,
        title: String,
        content: Option<String>,
        description: Option<String>,
        status: Option<Status>,
    ) -> Result<String, Error> {
        let db = RmORM::get_db();
        let mut product = Product::new_model(&db);
        product.title = title;
        product.description = description;
        product.status = status.unwrap_or_default();
        if content.is_some() {
            product.content = Some(Content {
                params: vec![],
                raw_content: content.unwrap(),
            });
        }
        let re = product.save().await?;
        Ok(re.inserted_id.to_string())
    }

    async fn update_product(&self ,
                            object_id: String,
                            title: Option<String> ,
                            content: Option<String>,
                            description: Option<String>,
                            status: Option<Status>,
    ) -> Result<String , Error>{
        let db = RmORM::get_db();
        let mut product = Product::new_model(&db);
        product._id = object_id.into();
        Ok("".into())
    }
}


