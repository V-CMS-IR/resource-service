use async_graphql::{InputObject, SimpleObject};
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::options::FindOptions;
use serde::{Deserialize, Serialize};
use spark_orm::Model;
use crate::app::types::{DateTime, Result};
use crate::app::util::Paginate;

#[Model(coll_name = "games")]
#[derive(SimpleObject, Serialize, Deserialize, Default, Debug)]
pub struct Game {
    pub title: String,
    pub slug: String,
    pub category_id: ObjectId,
    //TODO write these in resolvers
    pub release_date: Option<DateTime>,
    pub metas: String, // change this to hashmap,
    pub game_brief: String,

    //TODO must write this later in Complex resolvers
    // brand
    // platforms
    // products
    // related blogs
}

#[derive(InputObject, Serialize, Deserialize, Debug)]
pub struct GameInput {
    pub title: String,
    pub slug: String,
    pub category_id: ObjectId,
    pub release_date: Option<String>,
    pub metas: String, // change this to hashmap,
    pub game_brief: String,
}

impl Game {
    pub async fn get_games_by_category(category_id: impl Into<ObjectId>, paginate: Option<Paginate>) -> Result<Vec<Game>> {
        let game_model = Game::new_model(None);
        let query = doc! {
            "category_id" : {
                "$eq" : category_id.into()
            }
        };

        let mut options = None;

        if let Some(paginate) = paginate {
            let offset = paginate.get_offset() as u64;
            let op = FindOptions::builder()
                .limit(Some(paginate.limit as i64))
                .skip(Some(offset))
                .build();
            options = Some(op);
        }

        let games = game_model.find_and_collect(
            query,
            options,
        ).await?;

        let data: Vec<Game> = games.into_iter()
            .filter_map(|game| game.ok())
            .collect();
        Ok(data)
    }
}


