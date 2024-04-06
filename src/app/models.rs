pub mod product;

use std::env::var;
use product::{ProductMutation, ProductQuery};
use async_graphql::{Context, Error, Guard, MergedObject};
use async_graphql::async_trait::async_trait;
use serde_json::Value;
use crate::app::permissions::Permission;
use crate::server::middleware::Auth;

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

        let permission = &self.permission;
        let client = Auth::prepare_request(auth);
        
        let webserver_host = var("WEBSERVER_HOST").expect("the USERS_SERVICE_HOST is not set");
        let user_service_port = var("USERS_SERVICE_PORT").expect("the USERS_SERVICE_PORT is not set");
        let htp = format!("http://{webserver_host}:{user_service_port}/api/v1/authorize/can/{permission}");
        println!("The url {htp}");
        let re = client.get(
            htp
        ).send().await;
        return match re {
            Ok(res) => {
                // println!("The Response {:?} " , &res.text().await);
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

                    Ok(())
                } else {
                    return Err(Error::new("You don't have permission"));
                }
            
            }
            //TODO here must create a log 
            Err(_error) => {
                panic!("Can't send request to users service");
            }
        };


        // Ok(())
    }
}