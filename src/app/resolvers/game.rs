use async_graphql::{ComplexObject, Object};
use mongodb::bson::oid::ObjectId;
use crate::app::models::game::{Game, GameInput};
use crate::app::types::{DateTime, Result};
use mongodb::bson::{Bson, doc};
use mongodb::options::FindOptions;
use crate::app::models::category::Category;
use crate::app::util::{List, Paginate};
use crate::app::permissions::GamePermissions;
use crate::app::models::AuthorizeGuard;
use crate::app::models::brand::Brand;

#[derive(Default)]
pub struct GameQuery;

#[derive(Default)]
pub struct GameMutation;

#[Object]
impl GameQuery {
    pub async fn games(&self, #[graphql(default)] paginate: Paginate) -> Result<List<Game>> {
        let game_model = Game::new_model(None);
        let option = FindOptions::builder().skip(
            Some(paginate.get_offset())
        ).limit(
            paginate.limit as i64
        ).build();
        let data: Vec<Game> = game_model.find_and_collect(
            doc! {},
            option,
        ).await?.into_iter().filter_map(|x| x.ok()).collect();

        Ok(
            List::new(data).set_paginate(paginate)
        )
    }

    pub async fn game(&self, game_id: ObjectId) -> Result<Option<Game>> {
        let mut game_model = Game::new_model(None);
        let game = game_model.find_one(
            doc! {
                "_id": game_id
            },
            None,
        ).await?;

        if let Some(game) = game {
            return Ok(
                Some(game.take_inner())
            );
        }

        Ok(None)
    }
}

#[ComplexObject]
impl Game {
    pub async fn brands(&self) -> Result<List<Brand>> {
        let brand_model = Brand::new_model(None);

        let founded = brand_model.find_and_collect(
            doc! {
                "brand_id": self._id
            },
            None,
        ).await?;


        let brands: Vec<Brand> =
            founded.into_iter()
                .filter_map(|brand| brand.ok())
                .collect();
        Ok(
            List::new(brands)
        )
    }
}

#[Object]
impl GameMutation {
    #[graphql(guard = "AuthorizeGuard::new(GamePermissions::STORE) ")]
    pub async fn new_game(&self, data: GameInput) -> Result<Bson> {
        Game::store_update(None, data).await
    }
    #[graphql(guard = "AuthorizeGuard::new(GamePermissions::UPDATE) ")]
    pub async fn update_game(&self, game_id: ObjectId, data: GameInput) -> Result<Bson> {
        Game::store_update(Some(game_id), data).await
    }
}

impl Game {
    pub async fn store_update(id: Option<ObjectId>, data: GameInput) -> Result<Bson> {
        let mut game_model = Game::new_model(None);
        let mut category = Category::new_model(None);
        let mut brand_model = Brand::new_model(None);

        let _ = category.find_one(
            doc! {
                "_id": data.category_id
            },
            None,
        ).await.expect("Can't find the Category");
        for brand_id in &data.brands_id {
            let _ = brand_model.find_one(
                doc! {
                "_id": brand_id
            },
                None,
            ).await?.expect("Can't find the Brand");
        }

        if let Some(id) = id {
            game_model.find_one(
                doc! {
                    "_id" : id
                },
                None,
            ).await.expect("Can't find Game");
        }

        game_model.title = data.title;
        game_model.slug = data.slug;
        game_model.category_id = data.category_id;
        if let Some(datetime) = data.release_date {
            game_model.release_date = Some(DateTime::from(datetime));
        }
        game_model.metas = data.metas;
        game_model.game_brief = data.game_brief;
        game_model.brands_id = data.brands_id;
        // game_model
        let insert_update_id = game_model.save(None).await?;
        Ok(insert_update_id)
    }
}