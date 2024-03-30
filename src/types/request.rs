use std::collections::HashMap;
use serde::{Deserialize};
use serde_json::Value;

#[derive(Deserialize , Debug)]
pub struct JsonAPISchema
{
    status: bool,
    data: Value,
    errors: HashMap<String, String>,
    timestamp: String,
}