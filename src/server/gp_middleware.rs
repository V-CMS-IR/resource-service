use std::sync::Arc;
use async_graphql::{async_trait, Request, ServerResult};
use async_graphql::extensions::{Extension, ExtensionContext, ExtensionFactory, NextPrepareRequest};

pub(super) struct GraphQlLifeCycle;

#[derive(Debug)]
pub struct AuthInfo {
    pub host_name: String,
}

#[async_trait::async_trait]
impl Extension for GraphQlLifeCycle {
    async fn prepare_request(
        &self,
        ctx: &ExtensionContext<'_>,
        request: Request,
        next: NextPrepareRequest<'_>,
    ) -> ServerResult<Request> {
        next.run(ctx, request).await
    }
}

#[async_trait::async_trait]
impl ExtensionFactory for GraphQlLifeCycle {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(GraphQlLifeCycle)
    }
}
