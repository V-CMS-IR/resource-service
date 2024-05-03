use std::sync::Arc;
use async_graphql::{Enum, Error, Object, Result, SimpleObject};
use mongodb::bson::doc;
use spark_orm::{Model, ProxyModelCrud, Spark};
use spark_orm::model::Prototype;
use serde::{Deserialize, Serialize};
use spark_orm::model::utility::inner_utility::InnerUtility;
use super::{AuthorizeGuard};
use crate::app::permissions::ProductP;
use crate::types::{DateWrapper, ObjectID};

//TODO Write A Comment For all section of it and separate Sections
type Creator = i16;
type Price = f32;

// #[derive(Model, Default, SimpleObject, Debug, Serialize, Deserialize)]
// #[coll_name = "Products"]
#[Model(coll_name = "Products")]
#[derive(SimpleObject, Serialize, Deserialize, Default, Debug)]
pub struct Product {
    #[serde(skip_serializing_if = "ObjectID::is_none")]
    _id: ObjectID,
    pub title: String,
    pub description: Option<String>,
    pub content: Option<Content>,
    pub status: Status,
    pub meta: Option<Vec<Meta>>,
    pub author: Creator,
    #[serde(default)]
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

#[derive(Default)]
pub struct ProductQuery;

#[derive(Default)]
pub struct ProductMutation;

#[Object]
impl ProductQuery {
    async fn product(&self, object_id: Option<ObjectID>) -> Result<Option<Product>, Error> {
        let db = Spark::get_db();
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
    async fn products(&self) -> Result<Vec<Product>, Error> {
        let db = Spark::get_db();
        let product = Product::new_model(&db);
        let sample = Prototype::Doc(
            doc! {
                "author": 0
            }
        );
        let mut re = Vec::new();
        let _find = product.find_with_callback(sample, |pr| {
            re.push(pr);
        }).await;
        println!("the count : {} ", re.len());
        Ok(re)
    }
}

#[Object]
impl ProductMutation {
    #[graphql(guard = "AuthorizeGuard::new(ProductP::STORE) ")]
    async fn new_product<'a>(
        &self,
        title: String,
        content: Option<String>,
        description: Option<String>,
        status: Option<Status>,
        price: Price,
    ) -> Result<String, Error> {
        let db = Spark::get_db();
        let mut product = Product::new_model(&db);
        product.title = title;
        product.description = description;
        product.status = status.unwrap_or_default();
        product.price = price;
        if content.is_some() {
            product.content = Some(Content {
                params: vec![],
                raw_content: content.unwrap(),
            });
        }
        let re = product.save().await?;
        Ok(re.inserted_id.to_string())
    }

    #[graphql(guard = "AuthorizeGuard::new(ProductP::UPDATE)")]
    async fn update_product(&self,
                            object_id: ObjectID,
                            title: Option<String>,
                            content: Option<String>,
                            description: Option<String>,
                            status: Option<Status>,
                            price: Price,
    ) -> Result<u64, Error> {
        let db = Spark::get_db();
        let mut product = Product::new_model(&db);
        let id = object_id.0;
        println!("THE ID {id:?}");

        if let Some(founded_product) = product.find_one(Prototype::Doc(
           doc! {
               "_id" : id
           }
        )).await? {

            product.fill(founded_product);
            if title.is_some() {
                product.title = title.unwrap();
            }
            println!("THE CONTENT {:?} " , content.is_some());
            if content.is_some() {
                product.content = Some(
                    Content {
                        raw_content: content.unwrap(),
                        params: vec![],
                    }
                );
            }
            product.description = description;
            if status.is_some() {
                product.status = status.unwrap();
            }
            product.price = price;
            return Ok(product.update().await?)
        }

        Err(
            Error{
                message: "Can't find product to update".into(),
                source: None,
                extensions: None,
            }
        )
    }

    #[graphql(guard = "AuthorizeGuard::new(ProductP::DELETE)")]
    async fn delete_product(&self, object_id: String) -> Result<String, Error> {
        let db = Spark::get_db();
        let mut product = Product::new_model(&db);
        product._id = object_id.into();
        product.delete().await?;
        Ok("post deleted".into())
    }
}


