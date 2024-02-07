use std::str::FromStr;
use async_graphql::scalar;
use mongodb::bson::{Bson, DateTime};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize , Deserialize , Debug)]
pub struct DateWrapper(DateTime);

impl Default for DateWrapper{
    fn default() -> Self {
        DateWrapper(DateTime::now())
    }
}
scalar!(DateWrapper);

#[derive(Serialize , Deserialize , Debug , Default)]
pub struct ObjectID(pub Option<ObjectId>);
scalar!(ObjectID);

impl From<String> for ObjectID{
    fn from(value: String) -> Self {

        let ob_id =mongodb::bson::oid::ObjectId::from_str(&value);
        if ob_id.is_ok(){
            return ObjectID(
                Some(Result::unwrap(ob_id))
            );
        }
        ObjectID(
            None
        )
    }
}
impl ObjectID{
    pub fn is_none(val : &ObjectID)->bool{
        val.0.is_none()
    }
}

