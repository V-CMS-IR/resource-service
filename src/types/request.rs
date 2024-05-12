use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, SimpleObject)]
pub struct Paginate {
    pub page: usize,
    pub total: usize,
}

#[derive(Debug, Default, Serialize, Deserialize, SimpleObject)]
pub struct MetaData {
    pub pagination: Paginate,
}