use async_graphql::Object;
use crate::app::types::Result;
use crate::app::models::brand::{Brand, BrandInput};
use mongodb::bson::{Bson, doc};
use mongodb::bson::oid::ObjectId;
use mongodb::options::FindOptions;
use crate::app::permissions::BrandPermissions;
use crate::app::models::AuthorizeGuard;
use crate::app::util::{List, Paginate};

#[derive(Default)]
pub struct BrandQuery;

#[derive(Default)]
pub struct BrandMutation;


#[Object]
impl BrandQuery {
    pub async fn brand(&self, brand_id: ObjectId) -> Result<Option<Brand>> {
        let mut brand_model = Brand::new_model(None);
        let founded = brand_model.find_one(
            doc! {
                "_id" : brand_id
            },
            None,
        ).await?;
        if let Some(brand) = founded {
            return Ok(
                Some(
                    brand.take_inner()
                )
            );
        }

        return Ok(None);
    }

    pub async fn brands(&self, #[graphql(default)] paginate: Paginate) -> Result<List<Brand>> {
        let game_model = Brand::new_model(None);
        let find_options = FindOptions::builder().limit(
            paginate.limit as i64
        ).skip(
            Some(paginate.get_offset())
        ).build();

        let founded = game_model.find_and_collect(
            doc! {},
            find_options,
        ).await?;
        let brands: Vec<Brand> = founded.into_iter()
            .flat_map(|brand| brand.ok())
            .collect();

        Ok(
            List::new(brands).set_paginate(paginate)
        )
    }
}

#[Object]
impl BrandMutation {
    #[graphql(guard = "AuthorizeGuard::new(BrandPermissions::UPDATE) ")]
    pub async fn new_brand(&self, data: BrandInput) -> Result<Bson> {
        Brand::store_update(None, data).await
    }

    #[graphql(guard = "AuthorizeGuard::new(BrandPermissions::UPDATE) ")]
    pub async fn update_brand(&self, brand_id: ObjectId, data: BrandInput) -> Result<Bson> {
        Brand::store_update(Some(brand_id), data).await
    }
}

impl Brand {
    pub async fn store_update(id: Option<ObjectId>, data: BrandInput) -> Result<Bson> {
        let mut brand_model = Brand::new_model(None);
        if let Some(id) = id {
            brand_model.find_one(
                doc! {
                    "_id": id
                },
                None,
            ).await.expect("Can't find Brand");
        }
        brand_model.title = data.title;
        brand_model.slug = data.slug;

        let id = brand_model.save(None).await?;

        Ok(id)
    }
}