use async_graphql::SimpleObject;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use spark_orm::Model;

#[Model(coll_name="games")]
#[derive(SimpleObject , Serialize , Deserialize , Default , Debug)]
pub struct Game{
    pub title: String,
    pub slug: String,
    pub category_id: ObjectId,
    pub brands_id: Vec<ObjectId>,
}