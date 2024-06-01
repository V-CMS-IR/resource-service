use async_graphql::Object;
use async_graphql::Error;
use crate::app::models::brand::Brand;
use mongodb::bson::Bson;
use mongodb::bson::oid::ObjectId;
use crate::types::ObjectID;

#[derive(Default)]
pub struct BrandQuery;

#[derive(Default)]
pub struct BrandMutation;


#[Object]
impl BrandQuery {
    pub async fn brand(&self) -> async_graphql::Result<String, Error> {
        Ok("df".to_string())
    }
}

#[Object]
impl BrandMutation {
    pub async fn new_brand(&self, title: String) -> Result<Bson, Error> {
        let mut brand_model = Brand::new_model(None);
        brand_model.title = title;
        let result = brand_model.save(None).await?;
        Ok(result)
    }

    pub async fn update_brand(&self, id: ObjectId, title: String) -> Result<Bson, Error> {
        let mut brand_model = Brand::new_model(None);
        brand_model.title = title;
        let result = brand_model.save(None).await?;

        Ok(result)
    }
}