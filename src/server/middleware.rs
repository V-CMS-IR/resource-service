use async_graphql::{ServerError, Result, Response, Pos , Schema , EmptyMutation};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::{State};
use axum::headers::HeaderMap;
use crate::server::AppState;
use crate::data_base::{DB, DBConfig};
use crate::models::{MainMutation, MainQuery};
use crate::models::product::Product;


fn get_value_from_headers(key: &str, headers: &HeaderMap) -> Option<String> {
    match headers.get(key)?.to_str() {
        Ok(value) => Some(String::from(value)),
        Err(_) => None
    }
}

/// this method select the database by host of user
/// if the host doesn't exists returns error
pub async fn specify_db(
    State(mut app_state): State<AppState>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.into_inner();
    if let Some(host_name) = get_value_from_headers("Host", &headers) {
        if is_valid_data_base(&host_name, &app_state.db.valid_databases) {
            let db = DB(app_state.db.client.database(&host_name));
            req = req.data(db);
            return app_state.schema.execute(req).await.into();
        }
    }
    let host_invalid = ServerError::new("Invalid Host", Some(Pos {
        line: 0,
        column: 0,
    }));
    let error_response = Response::from_errors(vec![host_invalid]);
    return GraphQLResponse::from(error_response);
}

fn is_valid_data_base(db_name: &String, db_lists: &Vec<String>) -> bool {
    // TODO db list must have update by event or route
    return match db_lists.iter().find(|&x| x == db_name) {
        Some(_) => true,
        None => false,
    }

}
