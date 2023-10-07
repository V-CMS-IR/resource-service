pub mod product;
use async_graphql::{Object, Result,};

pub struct MainQuery;
pub struct MainMutation;
#[Object]
impl MainQuery{
    async fn star(&self) -> Result<&str> {
        // let name = Episode::getName();
        Ok("ho")
    }
}


