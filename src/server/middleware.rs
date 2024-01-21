use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::State;
use axum::http::HeaderMap;
use mongodb::Database;

use crate::server::AppState;

/// this method select the database by host of user
/// if the host doesn't exists returns error
pub async fn specify_db(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.into_inner();
    // match get_data_base(&headers) {
    //     Ok(db) => {
    //         req = req.data(db);
    //         app_state.schema.execute(req).await.into()
    //     }
    //     Err(message) => {
    //         let error = ServerError::new(message, Some(Pos { line: 0, column: 0 }));
    //         let error_response = Response::from_errors(vec![error]);
    //         GraphQLResponse::from(error_response)
    //     }
    // }
    app_state.schema.execute(req).await.into()

}

fn is_valid_data_base(db_name: &String, valid_data_bases: &[String]) -> bool {
    valid_data_bases.iter().any(|x| x == db_name)
}

fn get_value_from_headers(key: &str, headers: &HeaderMap) -> String {
    match headers.get(key) {
        Some(value) => {
            if let Ok(value_str) = value.to_str() {
                value_str.to_string()
            } else {
                "".to_string() // Couldn't convert the header value to a string
            }
        }
        None => "".to_string(), // The key is not in the headers
    }
}

