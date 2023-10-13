use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::State;
use axum::headers::HeaderMap;
use async_graphql::{Error, FieldResult};
use crate::server::AppState;
use crate::data_base::DB;

fn get_value_from_headers(key: &str, headers: &HeaderMap) -> Option<String> {
    match headers.get(key)?.to_str() {
        Ok(value) => Some(String::from(value)),
        Err(_) => None
    }
}

pub async fn specify_db(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.into_inner();

    if let Some(host_name) = get_value_from_headers("Host", &headers) {
        let db = DB(app_state.db.client.database(&host_name));
        req = req.data(db);
    }
    app_state.schema.execute(req).await.into()
}


