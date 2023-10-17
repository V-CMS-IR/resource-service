use std::sync::Arc;
use async_graphql::{async_trait, Request, Response, ServerResult};
use async_graphql::extensions::{Extension, ExtensionContext, ExtensionFactory, NextPrepareRequest, NextRequest};
use crate::data_base::DB;

pub(super) struct GraphQlLifeCycle;


#[async_trait::async_trait]
impl Extension for GraphQlLifeCycle {
    async fn request(&self, ctx: &ExtensionContext<'_>, next: NextRequest<'_>) -> Response {
        next.run(ctx).await
    }
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
