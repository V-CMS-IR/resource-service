use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, SimpleObject)]
pub struct List<D>
where
    D: Sync,
    D: Send,
    D: async_graphql::OutputType
{
    pub data: D,
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

