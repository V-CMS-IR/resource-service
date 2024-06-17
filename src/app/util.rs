use async_graphql::{InputObject, OutputType, SimpleObject};
use serde::{Deserialize, Serialize};
use crate::app::models::category::Category;
use crate::app::models::product::Product;
use crate::app::models::game::Game;

#[derive(Debug, Default, Serialize, Deserialize, SimpleObject)]
#[graphql(concrete(name = "categories", params(Category)))]
#[graphql(concrete(name = "products", params(Product)))]
#[graphql(concrete(name = "games", params(Game)))]
pub struct List<D>
    where
        D: Sync,
        D: Send,
        D: OutputType
{
    pub data: Vec<D>,
    pub meta_data: MetaData,
}

#[derive(Debug, Default, Serialize, Deserialize, SimpleObject)]
pub struct MetaData {
    pub pagination: Paginate,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(input_name = "InputPaginate")]
pub struct Paginate {
    pub page: usize,
    pub total: usize,
    pub limit: usize,
}

impl<D> List<D>
    where
        D: Sync,
        D: Send,
        D: OutputType
{
    pub fn new(data: Vec<D>) -> List<D> {
        List {
            data,
            meta_data: MetaData {
                pagination: Paginate::new(0, 0, 0)
            },
        }
    }

    pub fn set_paginate(&mut self , paginate: Paginate){
        self.meta_data.pagination = paginate;
    }
}


impl Paginate {
    pub fn new(total: usize, page: usize, limit: usize) -> Paginate {
        Paginate {
            total,
            page,
            limit,
        }
    }

    pub fn get_offset(&self) -> usize {
        (self.page - 1) * self.limit
    }
}


impl Default for Paginate {
    fn default() -> Self {
        Paginate::new(0, 1, 15)
    }
}