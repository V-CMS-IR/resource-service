use std::str::FromStr;
use async_graphql::{Context, Object};
use async_graphql::Error;
use mongodb::bson::{doc};
use mongodb::bson::oid::ObjectId;
use mongodb::options::FindOneOptions;
use crate::app::models::category::Category;
use crate::app::models::AuthorizeGuard;
use crate::app::permissions::CategoryPermissions;

#[derive(Default)]
pub struct CategoryQuery;

#[derive(Default)]
pub struct CategoryMutation;

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

    pub async fn categories(&self, _ctx: &Context<'_>) -> Result<String, Error> {
        Ok("difjo".to_string())
    }
}

#[Object]
impl CategoryMutation {
    #[graphql(guard = "AuthorizeGuard::new(CategoryPermissions::STORE) ")]
    pub async fn new_category(&self, title: String, slug: Option<String>) -> Result<String, Error> {
        Category::store_update_category(None, title, slug).await
    }

    #[graphql(guard = "AuthorizeGuard::new(CategoryPermissions::UPDATE) ")]
    pub async fn update_category(&self, id: String, title: String, slug: Option<String>) -> Result<String, Error> {
        Category::store_update_category(Some(id), title, slug).await
    }
}

impl Category {
    async fn store_update_category(id: Option<String>, title: String, slug: Option<String>)
                                   -> Result<String, Error>
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
            id.to_string()
        )
    }
}


