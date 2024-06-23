use async_graphql::{Error, Object};
use mongodb::bson::{Bson, doc};
use mongodb::bson::oid::ObjectId;
use mongodb::options::FindOptions;
use spark_orm::Spark;
use crate::app::models::product::{Product, ProductInput};
use crate::types::ObjectID;
use crate::app::permissions::ProductPermissions;
use crate::app::models::AuthorizeGuard;
use crate::app::models::brand::Brand;
use crate::app::util::{List, MetaData, Paginate};
use crate::app::types::Result;

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
    async fn products(&self, #[graphql(default = 1)] page: usize, #[graphql(
        default = 15
    )]limit: usize)
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
            .filter_map(|product| product.ok()) // Filter out Err variants and unwrap Ok variants
            .collect();
        Ok(
            List {
                data: unwrapped_founded,
                meta_data: MetaData {
                    pagination: Paginate {
                        page,
                        total,
                        limit,
                    }
                },
            }
        )
        // Ok(re)
    }
}
#[Object]
impl ProductMutation {
    #[graphql(guard = "AuthorizeGuard::new(ProductPermissions::STORE) ")]
    async fn new_product<'a>(&self, data: ProductInput) -> Result<Bson> {
        Product::store_update(None, data).await
    }

    #[graphql(guard = "AuthorizeGuard::new(ProductPermissions::UPDATE)")]
    async fn update_product(&self, product_id: ObjectId, data: ProductInput) -> Result<Bson> {
        Product::store_update(Some(product_id), data).await
    }

    #[graphql(guard = "AuthorizeGuard::new(ProductPermissions::DELETE)")]
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


impl Product {
    pub async fn store_update(id: Option<ObjectId>, data: ProductInput) -> Result<Bson> {
        let mut product_model = Product::new_model(None);
        let mut brand_model = Brand::new_model(None);

        if let Some(id) = id {
            product_model._id = Some(id);
        }

        brand_model.find_one(
            doc! {
                "_id" : data.brand_id
            },
            None,
        ).await.expect("Brand not exists");

        product_model.title = data.title;
        product_model.description = data.description;
        product_model.content = data.content;
        product_model.status = data.status;
        product_model.meta = data.meta;
        product_model.price = data.price;
        product_model.brand_id = data.brand_id;

        let re = product_model.save(None).await?;
        Ok(re)
    }
}

