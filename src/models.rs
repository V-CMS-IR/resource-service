pub mod product;
use async_graphql::{MergedObject, Response};
use crate::models::product::{ProductMutation, ProductQuery};
use async_graphql::extensions::{Extension, ExtensionContext, NextRequest};


#[derive(MergedObject , Default)]
pub struct MainQuery(ProductQuery);
#[derive(MergedObject , Default)]
pub struct MainMutation(ProductMutation);



