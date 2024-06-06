use async_graphql::SimpleObject;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::options::FindOptions;
use serde::{Deserialize, Serialize};
use spark_orm::Model;
use crate::app::types::Result;
use crate::app::util::Paginate;

#[Model(coll_name = "games")]
#[derive(SimpleObject, Serialize, Deserialize, Default, Debug)]
pub struct Game {
    pub title: String,
    pub slug: String,
    pub category_id: ObjectId,
    pub brands_id: Vec<ObjectId>,
}


impl Game {
    pub async fn get_games(ids: Option<&Vec<ObjectId>>, paginate: Option<Paginate>) -> Result<Vec<Game>> {
        let game_model = Game::new_model(None);
        let mut query = doc! {};
        let mut options = None;

        if let Some(ids) = ids {
            query = doc! {
                   "_id": {
                       "$in": ids
                   }
               }
        }
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