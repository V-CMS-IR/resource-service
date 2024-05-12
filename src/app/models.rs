pub mod product;
mod category;

use std::env::var;
use async_graphql::{Context, Error, Guard, MergedObject};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::app::permissions::Permission;
use crate::server::middleware::Auth;
use crate::app::resolvers::product::{ProductMutation, ProductQuery};
#[derive(MergedObject, Default)]
pub struct MainQuery(
    ProductQuery

);

#[derive(MergedObject, Default)]
pub struct MainMutation(
    ProductMutation

);

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

//TODO write a cfg or env for this shity macro
impl<P:Permission> Guard for AuthorizeGuard<P>
    where P:Send , P:Sync
{

    async fn check(&self , ctx: &Context<'_>)-> async_graphql::Result<()> {
        let auth = ctx.data::<Auth>().unwrap();

        let permission = &self.permission;
        let client = Auth::prepare_request(auth);

        let webserver_host = var("WEBSERVER_HOST").expect("the USERS_SERVICE_HOST is not set");
        let url = format!("http://{webserver_host}:/api/v1/authorize/can/{permission}");
        let response = client.get(
            url
        ).send().await;

        return match response {
            Ok(res) => {
                if res.status().is_success() {
                    let body = &res.json::<Value>().await.unwrap();

                    let status = body.get("status").unwrap().as_bool().unwrap();

                    if !status {
                        // TODO add debug support to errors message
                        return Err(
                            Error::new(
                                body.get("errors").expect("errors field must be exists ").as_str().expect("")
                            )
                        );
                    }
                    //TODO replace empty unwrap 
                    let data = body.get("data").unwrap().as_object().unwrap();
                    let can = data.get("can").unwrap().as_bool().unwrap();

                    if !can {
                        return Err(
                            Error::new(
                                "Can't act in this section"
                            )
                        );
                    }
                    return Ok(());
                }
                Err(
                   Error::new(
                       format!("The Status {} and message is {}",
                               res.status().as_str(),
                               &res.text().await.unwrap().as_str()
                       )
                   )
                )
            },
            Err(error) => {
                Err(
                    error.into()
                )
            }
        }
    }
}


