use std::str::FromStr;
use async_graphql::{ComplexObject, Object};
use async_graphql::Error;
use mongodb::bson::{Bson, doc};
use mongodb::bson::oid::ObjectId;
use mongodb::options::{FindOptions};
use crate::app::models::category::Category;
use crate::app::models::AuthorizeGuard;
use crate::app::permissions::CategoryPermissions;
use crate::app::util::{List, MetaData, Paginate};
use crate::types::ObjectID;

#[derive(Default)]
pub struct CategoryQuery;

#[derive(Default)]
pub struct CategoryMutation;

type CategoryList = List<Category>;

#[Object]
impl CategoryQuery {
    pub async fn category(&self, id: String) -> Result<Option<Category>, Error> {
        let mut category_model = Category::new_model(None);
        let id = ObjectId::from_str(&id)?;

        let re = category_model.find_one(
            doc! {
             "_id" : id
           },
            None,
        ).await?;
        if let Some(model) = re {
            return Ok(
                Some(model.take_inner())
            );
        }

        Ok(None)
    }

    pub async fn categories(&self, #[graphql(default = 1)] page: usize, #[graphql(default = 15)]limit: usize)
                            -> Result<CategoryList, Error> {
        let category_model = Category::new_model(None);

        let offset = (page - 1) * limit;
        // TODO you must get the author from the users service
        let sample = doc! {};
        let options = FindOptions::builder()
            .skip(Some(offset as u64))
            .limit(Some(limit as i64)).build();
        let founded = category_model.find_and_collect(
            sample.clone(),
            Some(options),
        ).await?;

        let total = category_model.find_and_collect(
            sample,
            None,
            // Some(FindOptions::builder().projection(Some(doc! {"id" : ""})).build())
        ).await?.iter().count();

        let unwrapped_founded: Vec<Category> = founded
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
                    }
                },
            }
        )
    }
}

#[Object]
impl CategoryMutation {
    #[graphql(guard = "AuthorizeGuard::new(CategoryPermissions::STORE) ")]
    pub async fn new_category(&self, title: String, slug: Option<String>) -> Result<Bson, Error> {
        Category::store_update_category(None, title, slug).await
    }

    #[graphql(guard = "AuthorizeGuard::new(CategoryPermissions::UPDATE) ")]
    pub async fn update_category(&self, id: String, title: String, slug: Option<String>) -> Result<Bson, Error> {
        Category::store_update_category(Some(id), title, slug).await
    }

    #[graphql(guard = "AuthorizeGuard::new(CategoryPermissions::DELETE) ")]
    pub async fn delete_category(&self, id: ObjectID) -> Result<u64, Error> {
        let category_model = Category::new_model(None);
        let re = category_model.delete(
            doc! {
                "_id": id.0
            },
            None,
        ).await?;
        Ok(re)
    }
}

#[ComplexObject]
impl Category {
    pub async fn products(&self) -> Result<String, Error> {
        //TODO here must fetch the products
        Ok("NOT IMPLEMENTED YET".to_string())
    }
}

impl Category {
    async fn store_update_category(id: Option<String>, title: String, slug: Option<String>)
                                   -> Result<Bson, Error>
    {
        let mut category_model = Category::new_model(None);
        if id.is_some() {
            category_model._id = Some(ObjectId::from_str(&id.unwrap())?);
        }
        let slug = if slug.is_some() {
            slug.unwrap()
        } else {
            title.clone()
        };
        category_model.title = title;
        category_model.slug = slug.into();
        let id = category_model.save(None).await?;
        Ok(
            id
        )
    }
}


