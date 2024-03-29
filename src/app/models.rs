pub mod product;

use std::fmt::{Display, Pointer};
use product::{ProductMutation, ProductQuery};
use async_graphql::{Context, Guard, MergedObject};
use async_graphql::async_trait::async_trait;
use reqwest::Client;
use crate::app::permissions::Permission;
use crate::server::middleware::Auth;
use reqwest::header::{HeaderMap, HeaderValue};

#[derive(MergedObject, Default)]
pub struct MainQuery(ProductQuery);

#[derive(MergedObject, Default)]
pub struct MainMutation(ProductMutation);

pub struct AuthorizeGuard<P: Permission> where
    P: Sync,
    P: Send
{
    permission: P,
}

impl<P: Permission> AuthorizeGuard<P>
    where
        P: Sync,
        P: Send
{
    pub fn new(permission: P) -> Self {
        Self {
            permission
        }
    }


}


#[async_trait]
impl<P: Permission> Guard for AuthorizeGuard<P>
    where
        P: Send,
        P: Sync
{
    async fn check(&self, ctx: &Context<'_>) -> async_graphql::Result<()> {
        let auth = ctx.data::<Auth>().unwrap();

        let p = &self.permission;
        let client = Auth::prepare_request(&auth) ;
        let re = client.get(
            format!("http://localhost:8000/api/v1/authorize/can/{p}")
        ).send().await;
        Ok(())
    }
}