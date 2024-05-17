use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::State;
use axum::http::{HeaderMap, HeaderValue};
use reqwest::Client;

use crate::server::AppState;

pub struct Auth {
    pub authentication_token: String
}

impl Auth{
    pub(crate) fn prepare_request(auth: &Auth) -> Client {
        let client = Client::builder();
        let mut headers = HeaderMap::default();
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&auth.authentication_token).unwrap(),
        );
        headers.insert(
            "Accept",
            HeaderValue::from_str("application/json").unwrap(),
        );
        client.default_headers(headers).build().unwrap()
    }
}

/// this method select the database by host of user
/// if the host doesn't exist returns error
pub async fn execute_gql(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.into_inner();
    
    // auth section 
    let token = get_value_from_headers("Authorization" , &headers).unwrap_or_default();
    let auth = Auth{
        authentication_token: token
    };
    req = req.data(auth);





    
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
#[allow(dead_code)]
fn is_valid_data_base(db_name: &String, valid_data_bases: &[String]) -> bool {
    valid_data_bases.iter().any(|x| x == db_name)
}

fn get_value_from_headers(key: &str, headers: &HeaderMap) -> Result<String , String> {
    match headers.get(key) {
        Some(value) => {
            if let Ok(value_str) = value.to_str() {
                Ok(value_str.to_string())
            } else {
                Ok("".to_string()) // Couldn't convert the header value to a string
            }
        }
        None => Err(
            format!("The {key} in header is missing")
        ) // The key is not in the headers
    }
}

