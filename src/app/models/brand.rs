use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use spark_orm::Model;

#[Model(coll_name = "brands")]
#[derive(SimpleObject, Serialize, Deserialize, Default, Debug)]
pub struct Brand {
    pub title: String,
    pub slug: String,

    // complex resolvers
    // pub products
}

#[derive(InputObject, Serialize, Deserialize, Debug)]
pub struct BrandInput {
    pub title: String,
    pub slug: String,
}
