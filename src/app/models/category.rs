use async_graphql::SimpleObject;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use spark_orm::Model;

#[Model(coll_name="categories")]
#[derive(SimpleObject , Serialize , Deserialize , Default ,  Debug)]
#[graphql(complex)]
pub struct Category {
    pub slug: String,
    pub title: String,
    #[graphql(skip)]
    pub games_id: Vec<ObjectId>,

    // we resolve the games with custom resolver
}
