use std::str::FromStr;
use async_graphql::{Error, Object};
use mongodb::bson::{doc, to_document};
use mongodb::bson::oid::ObjectId;
use mongodb::options::FindOptions;
use spark_orm::Spark;
use crate::app::models::product::{Content, Product, Status};
use crate::types::ObjectID;
use crate::app::permissions::ProductP;
use crate::app::models::AuthorizeGuard;
use crate::app::models::category::Category;
use crate::app::util::{List, MetaData, Paginate};

#[derive(Default)]
pub struct ProductQuery;

#[derive(Default)]
pub struct ProductMutation;

#[Object]
impl ProductQuery {
    async fn product(&self, object_id: Option<ObjectID>) -> async_graphql::Result<Option<Product>, Error> {
        let db = Spark::get_db();
        let mut product = Product::new_model(Some(&db));

        // TODO you must get it from the users service
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
        if let Some(pr) = re {
            let rt = pr.take_inner();
            return Ok(
                Some(
                    rt
                )
            );
        }
        Ok(None)
    }
    async fn products(&self, #[graphql(default = 1)] page: usize, #[graphql(default = 15)]limit: usize)
                      -> async_graphql::Result<List<Product>, Error> {
        let db = Spark::get_db();
        let product = Product::new_model(Some(&db));

        let offset = (page - 1) * limit;
        // TODO you must get the author from the users service
        let sample = doc! {
                "author": 0,
            };
        let options = FindOptions::builder()
            .skip(Some(offset as u64))
            .limit(Some(limit as i64)).build();
        let founded = product.find_and_collect(
            sample.clone(),
            Some(options),
        ).await?;

        let total = product.find_and_collect(
            sample,
            None,
            // Some(FindOptions::builder().projection(Some(doc! {"id" : ""})).build())
        ).await?.iter().count();

        let unwrapped_founded: Vec<Product> = founded
            .into_iter()
            .filter_map(Result::ok) // Filter out Err variants and unwrap Ok variants
            .collect();
        Ok(
            List {
                data: unwrapped_founded,
                meta_data: MetaData {
                    pagination: Paginate {
                        page,
                        total,
                        limit
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
        category_id: String,
        title: String,
        content: Option<String>,
        description: Option<String>,
        status: Option<Status>,
        price: crate::app::models::product::Price,
    ) -> async_graphql::Result<String, Error> {
        let mut product = Product::new_model(None);
        let mut category_model = Category::new_model(None);
        product.title = title;

        //this is a key to a category
        let c_id = ObjectId::from_str(&category_id)?;
        let f_c = category_model.find_one(
            doc! {
               "_id" : c_id
            },
            None,
        );
        match f_c.await? {
            Some(category_model) => {
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
            None => {
                return Err(
                    Error::new(
                        "Can't find the category"
                    )
                );
            }
        }
    }

    #[graphql(guard = "AuthorizeGuard::new(ProductP::UPDATE)")]
    async fn update_product(&self,
                            id: ObjectID,
                            category_id: ObjectID,
                            title: Option<String>,
                            content: Option<String>,
                            description: Option<String>,
                            status: Option<Status>,
                            price: crate::app::models::product::Price,
    ) -> async_graphql::Result<u64, Error> {
        let mut product = Product::new_model(None);

        if category_id.0.is_some() {
            let mut category_model = Category::new_model(None);
            let f_c = category_model.find_one(
                doc! {
               "_id" : category_id.0.unwrap()
            },
                None,
            ).await?;
        }

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
        let doc = to_document(&product.take_inner())?;
        let result = product.update(
            doc! {
                "_id": id.0
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
    async fn delete_product(&self, object_id: ObjectID) -> async_graphql::Result<String, Error> {
        let db = Spark::get_db();
        let product = Product::new_model(Some(&db));
        let id = object_id.0;
        let re = product.delete(
            doc! {
              "_id" : id
            },
            None,
        ).await?;
        println!("THE RE {re}");
        Ok("post deleted".into())
    }
}

