use async_graphql::SimpleObject;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use spark_orm::Model;

#[Model(coll_name = "brands")]
#[derive(SimpleObject, Serialize, Deserialize, Default, Debug)]
pub struct Brand {
    pub title: String,
    pub slug: String,
    pub game_id: ObjectId,
    pub products_id: Vec<ObjectId>,
}