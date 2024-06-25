use async_graphql::Context;
use minio_rsc::Minio;
use crate::file_storage::FileStorage;
use crate::server::AppState;

pub(crate) fn minio<'a>(ctx: &Context<'a>) -> &'a FileStorage<Minio>{
    let state = ctx.data::<AppState>().expect("Can't find AppState Schema");
    &state.file_storage
}