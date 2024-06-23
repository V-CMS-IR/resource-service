#![allow(clippy::upper_case_acronyms)]

// use std::fmt::Display;
use strum::{Display, EnumString};

#[derive(EnumString , Display)]
#[strum(prefix="PRODUCT_")]
pub enum ProductPermissions {
    STORE,
    UPDATE,
    DELETE,
}

#[derive(EnumString , Display)]
#[strum(prefix="CATEGORY_")]
pub enum CategoryPermissions{
    STORE,
    UPDATE,
    DELETE
}

#[derive(EnumString , Display)]
#[strum(prefix="GAME_")]
pub enum GamePermissions{
    STORE,
    UPDATE,
    DELETE
}

#[derive(EnumString , Display)]
#[strum(prefix="BRAND_")]
pub enum BrandPermissions {
    STORE,
    UPDATE,
    DELETE
}