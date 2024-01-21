pub mod product;

use product::{ProductMutation, ProductQuery};
use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct MainQuery(ProductQuery);

#[derive(MergedObject, Default)]
pub struct MainMutation(ProductMutation);
