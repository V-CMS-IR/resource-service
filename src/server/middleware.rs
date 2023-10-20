use async_graphql::{ServerError, Response, Pos};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::{State};
use axum::headers::HeaderMap;
use mongodb::Database;
use crate::server::AppState;
use crate::global_data::GLOBAL_DATA;


/// this method select the database by host of user
/// if the host doesn't exists returns error
pub async fn specify_db(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.into_inner();
    return match get_data_base(&headers) {
        Ok(db) => {
            req = req.data(db);
            app_state.schema.execute(req).await.into()
        }
        Err(message) => {
            let error = ServerError::new(message, Some(Pos { line: 0, column: 0 }));
            let error_response = Response::from_errors(vec![error]);
            GraphQLResponse::from(error_response)
        }
    };
}

fn is_valid_data_base(db_name: &String, valid_data_bases: &Vec<String>) -> bool {
    match valid_data_bases.iter().find(|&x| x == db_name) {
        Some(_) => true,
        None => false
    }
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

fn get_data_base(headers: &HeaderMap) -> Result<Database, String> {
    let global_data = GLOBAL_DATA.try_lock();
    if let Ok(mutex_global_data) = global_data {
        if let Some(db_config) = mutex_global_data.get_db_config() {
            let client = db_config.get_client();
            let valid_data_bases = &db_config.valid_database;
            let db_name = get_value_from_headers("Host", &headers);
            return if is_valid_data_base(&db_name, valid_data_bases) {
                Ok(client.database(&db_name))
            } else {
                Err("Invalid Host ".to_string())
            };
        }
    }
    Err("Some thing went wrong #100500 ".to_string())
}