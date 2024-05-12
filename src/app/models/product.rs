use async_graphql::{Enum, Error, Object, Result, SimpleObject};
use mongodb::bson::{doc, to_document};
use mongodb::options::FindOptions;
use spark_orm::{Model, Spark};
use serde::{Deserialize, Serialize};
use super::{AuthorizeGuard};
use crate::app::permissions::ProductP;
use crate::types::{DateWrapper, ObjectID};
use crate::types::request::{MetaData, Paginate};

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
    pub price: Price,
    pub created_at: DateWrapper,
    pub updated_at: DateWrapper,
    pub deleted_at: DateWrapper,
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct ProductList {
    products: Vec<Product>,
    metadata: MetaData,
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
        let mut product = Product::new_model(Some(&db));
        let mut sample = doc! {
            "author": 0
        };
        if object_id.is_some() {
            let ob = object_id.unwrap().0;
            sample.insert("_id", ob);
        }
        let re = product.find_one(
            sample,
            None,
        ).await?;
        if let Some(mut pr) = re {
            let rt = pr.inner_deref();
            return Ok(
                Some(
                    rt
                )
            );
        }
        Ok(None)
    }
    async fn products(&self, #[graphql(default = 1)] page: usize, #[graphql(default = 15)]limit: usize) -> Result<ProductList, Error> {
        let db = Spark::get_db();
        let product = Product::new_model(Some(&db));

        let offset = (page - 1) * limit;
        let sample = doc! {
                "author": 0,
            };
        let options = FindOptions::builder()
            .skip(Some(offset as u64))
            .limit(Some(limit as i64)).build();
        let founded = product.find_and_collect(
            sample,
            Some(options),
        ).await?;
        let unwrapped_founded: Vec<Product> = founded
            .into_iter()
            .filter_map(Result::ok) // Filter out Err variants and unwrap Ok variants
            .collect();
        Ok(
            ProductList {
                products: unwrapped_founded,
                metadata: MetaData {
                    pagination: Paginate {
                        page,
                        total: 10,
                    }
                },
            }
        )
        // Ok(re)
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
        let mut product = Product::new_model(Some(&db));
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
        let re = product.save(None).await?;
        Ok(re.to_string())
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
        let mut product = Product::new_model(Some(&db));

        if title.is_some() {
            product.title = title.unwrap();
        }
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
        let id = object_id.0;
        let doc = to_document(&product.inner_deref())?;
        let result = product.update(
            doc! {
                "_id": id
            },
            doc! {
                "$set": doc
            }
            , None).await;
        if let Ok(re) = result {
            return Ok(re.modified_count);
        }

        println!("The Product Update error {:?} ", result);

        Err(
            Error {
                message: "Can't find product to update".into(),
                source: None,
                extensions: None,
            }
        )
    }

    #[graphql(guard = "AuthorizeGuard::new(ProductP::DELETE)")]
    async fn delete_product(&self, object_id: ObjectID) -> Result<String, Error> {
        let db = Spark::get_db();
        let mut product = Product::new_model(Some(&db));
        let id = object_id.0;
        let re = product.delete(
            doc!{
              "_id" : id
            },
            None
        ).await?;
        println!("THE RE {re}");
        Ok("post deleted".into())
    }
}


