pub mod product;
use async_graphql::{MergedObject};
use crate::models::product::ProductQuery;

#[derive(MergedObject , Default)]
pub struct MainQuery(ProductQuery);
#[derive(MergedObject , Default)]
pub struct MainMutation;




