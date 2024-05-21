use async_graphql::{OutputType, SimpleObject};
use serde::{Deserialize, Serialize};
use crate::app::models::category::Category;
use crate::app::models::product::Product;
#[derive(Debug, Default, Serialize, Deserialize, SimpleObject)]
#[graphql(concrete(name = "categories", params(Category)))]
#[graphql(concrete(name = "products", params(Product)))]
pub struct List<D>
    where
        D: Sync,
        D: Send,
        D: OutputType
{
    pub data: Vec<D>,
    pub meta_data: MetaData,
}

#[derive(Debug, Default, Serialize, Deserialize, SimpleObject)]
pub struct MetaData {
    pub pagination: Paginate,
}

#[derive(Debug, Default, Serialize, Deserialize, SimpleObject)]
pub struct Paginate {
    pub page: usize,
    pub total: usize,
}

