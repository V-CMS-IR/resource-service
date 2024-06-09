use async_graphql::Object;
use mongodb::bson::oid::ObjectId;
use crate::app::models::game::Game;
use crate::app::types::{Error, Result};
use mongodb::bson::{Bson, doc};
use mongodb::options::FindOptions;
use crate::app::models::category::Category;
use crate::app::util::{List, Paginate};

#[derive(Default)]
pub struct GameQuery;

#[derive(Default)]
pub struct GameMutation;

#[Object]
impl GameQuery {
    pub async fn games(&self, #[graphql(default)] paginate: Paginate) -> Result<List<Game>> {
        let mut game_model = Game::new_model(None);
        let option = FindOptions::builder().skip(
            Some(paginate.get_offset() as u64)
        ).limit(
            paginate.limit as i64
        ).build();
        let data: Vec<Game> = game_model.find_and_collect(
            doc! {},
            option,
        ).await?.into_iter().filter_map(|x| x.ok()).collect();
        let mut list = List::new(
            data
        );
        list.set_paginate(paginate);

        Ok(list)
    }
}

#[Object]
impl GameMutation {
    pub async fn new_game(&self, category_id: ObjectId, title: String, slug: String) -> Result<Bson> {
        let mut game_model = Game::new_model(None);
        let mut category = Category::new_model(None);
        let founded_cat = category.find_one(
            doc! {
                "_id": category_id
            },
            None,
        ).await?;

        if founded_cat.is_some() {
            game_model.title = title;
            game_model.slug = slug;
            game_model.category_id = category_id;
            let id = game_model.save(None).await?;
            return Ok(id);
        }

        return Err(
            Error::new("Can't find category")
        );
    }
}